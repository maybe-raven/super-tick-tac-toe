use std::{
    cell::{Ref, RefCell},
    iter::Map,
    ops::ControlFlow,
    rc::{Rc, Weak},
    slice,
};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use tracing::instrument;

use crate::{BoardIndex, BoardOutcome, BoardState, Game, MarkTileResult, Play, Player, Region};

use super::random::GenerateMove;

const EXPLORE_PARAM: f32 = std::f32::consts::SQRT_2;
const SCORE_WIN: f32 = 1.0;
const SCORE_DRAW: f32 = 0.5;
const SCORE_LOSS: f32 = 0.0;

#[instrument(skip(should_terminate, game))]
pub fn make_move(game: Game, should_terminate: impl Fn(&Node) -> bool) -> Play {
    assert!(matches!(game.state, BoardState::InProgress));

    let root = Node::new_root();
    let mut cursor = Cursor::new(Rc::clone(&root), game);

    loop {
        cursor.run();
        if should_terminate(&root.borrow()) {
            break;
        }
    }

    let root = root.borrow();
    let best_node = root
        .children
        .iter()
        .max_by(|a, b| a.borrow().score.total_cmp(&b.borrow().score))
        .expect("an in-progress game should always have at least one possible play.");

    let play = best_node
        .borrow()
        .play
        .expect("all nodes except the root should denote a play from the parent game state");
    play
}

type NodeRef = Rc<RefCell<Node>>;

/// A node in a Monte Carlo Tree
///
/// Each node (aside from the *root* node) stores a play that can be made in the game from its
/// parent state.
/// Each node represents a game state that can be reached by applying the play to its parent state
/// starting from the root.
///
/// A *terminal* node is a node that represents a game-over state.
///
/// A *leaf* node is a node with can be expanded (is not *terminal*) but hasn't been expanded yet,
/// so it has no children.
///
/// Usage:
/// Store the root node and navigate the tree with a [`Cursor`].
#[derive(Clone, Debug)]
pub struct Node {
    /// A *valid* play from the parent game state.
    ///
    /// Invariant: Should only be `None` for the root node,
    /// in other words: `self.play.is_none() == self.parent.is_none()`
    play: Option<Play>,
    /// The total score of all rollouts from this node and all its children.
    score: f32,
    /// The total number of rollouts from this node and all its children.
    n_visits: usize,
    /// The children of this nodes, representing all valid plays from the current game state.
    children: Vec<NodeRef>,
    /// The parent node.
    ///
    /// Invariants:
    /// 1. Should only be `None` for the root node
    /// 2. If it's `Some`, `parent.children` should contain this node.
    parent: Option<Weak<RefCell<Node>>>,
}

impl Node {
    /// Returns the total score of all rollouts from this node and all its children.
    pub fn score(&self) -> f32 {
        self.score
    }
    /// Returns the total number of rollouts from this node and all its children.
    pub fn n_visits(&self) -> usize {
        self.n_visits
    }
    /// Returns the total score devided by the total number of rollouts.
    pub fn average_score(&self) -> f32 {
        self.score / self.n_visits as f32
    }
    /// Returns an iterator over this node's children.
    pub fn children<'a>(
        &'a self,
    ) -> Map<slice::Iter<'a, NodeRef>, impl FnMut(&'a NodeRef) -> Ref<'a, Node>> {
        self.children.iter().map(|x: &NodeRef| x.borrow())
    }
    /// Returns a new root node.
    fn new_root() -> NodeRef {
        Rc::new(RefCell::new(Self {
            play: None,
            score: 0.0,
            n_visits: 0,
            children: Vec::new(),
            parent: None,
        }))
    }
    /// Adds a child node with the given `play` to `this` node.
    fn add_child(this: &NodeRef, play: Play) {
        let node = Self {
            play: Some(play),
            score: 0.0,
            n_visits: 0,
            children: Vec::new(),
            parent: Some(Rc::downgrade(this)),
        };
        this.borrow_mut().children.push(Rc::new(RefCell::new(node)));
    }
    /// Calculates the upper confidence bounds (UCB) of this node.
    ///
    /// `lnn` is the natural log of the total rollouts of the parent node,
    /// in other words: `lnn = (self.parent.upgrade().expect().n_visits as f32).ln()`
    /// This function takes `lnn` as a parameter to allow the caller to calculate it once and cache
    /// it, since this function is expected to be called repeatedly over every child of a node when
    /// searching for the best child.
    fn ucb1(&self, lnn: f32) -> f32 {
        if self.n_visits == 0 {
            return f32::INFINITY;
        }
        self.average_score() + EXPLORE_PARAM * (lnn / self.n_visits as f32).sqrt()
    }
    /// Updates the total score of this node by the given amount and increment the rollout counter.
    fn update_score(&mut self, score_update: f32) {
        self.score += score_update;
        self.n_visits += 1;
    }
    /// Returns the child with the highest UCB1 score or `None` if this node has no children.
    fn find_best_child(&self) -> Option<NodeRef> {
        let lnn = (self.n_visits as f32).ln();
        self.children
            .iter()
            .max_by(|node0: &&NodeRef, node1: &&NodeRef| {
                node0
                    .borrow()
                    .ucb1(lnn)
                    .total_cmp(&node1.borrow().ucb1(lnn))
            })
            .map(Rc::clone)
    }
    /// Returns `true` if the MCTS algorithm should skip the expansion step and perform a rollout
    /// immediately.
    ///
    /// This returns true for nodes that have had 0 rollouts.
    /// This will never be `true` for the root node.
    fn should_rollout(&self) -> bool {
        self.n_visits == 0 && self.play.is_some()
    }
}

/// A cursor for populating and navigating the Monte Carlo Tree.
///
/// The cursor holds an instance of [`Game`] and updates its state with the play stored at each
/// node as it traverses through the tree.
///
/// [`Cursor::run`] runs one iteration of the MCTS algorithm ending with backpropagating the score
/// up to the root.
///
/// This algorithm is implemented referencing this video:
/// https://www.youtube.com/watch?v=UXW2yZndl7U
struct Cursor {
    /// The root node of the tree.
    ///
    /// This is only stored to take shared ownership of the root, which then guarantees that no
    /// parts of the tree get dropped while the cursor is traversing it.
    _root_node: NodeRef,
    /// The node the cursor is currently pointing at.
    current_node: NodeRef,
    /// The player performing the move of the current node.
    current_player: Player,
    /// An instance of the original game state represented by the root node.
    ///
    /// This is needed since the cursor only keeps the game state updated as it traverses down the
    /// tree, but not while backpropagating up to the root. After backpropagation is finished, the
    /// cursor will clone this value to restore the game state.
    original_game: Game,
    /// The current game state represented by having just performed the play stored in
    /// `current_node`.
    game: Game,
    /// RNG for the random elements in the MCTS algorithm.
    rng: ThreadRng,
}

impl Cursor {
    /// Creates a new [`Cursor`] with the given root node and starting game state.
    fn new(root_node: NodeRef, game: Game) -> Self {
        Self {
            current_node: root_node.clone(),
            _root_node: root_node,
            current_player: game.current_player.other(),
            original_game: game.clone(),
            game,
            rng: thread_rng(),
        }
    }
    /// Updates the current game state by making the given play.
    ///
    /// The caller must ensure the given play is valid for the current game state.
    ///
    /// Returns `ControlFlow::Continue(())` if the game is still going after making the play;
    /// returns `ControlFlow::Break(outcome)` if the game ends.
    fn mark_tile(&mut self, play: Play) -> ControlFlow<BoardOutcome> {
        match self.game.mark_tile(play) {
            MarkTileResult::NoChange => unreachable!(
                "generated move should always be valid and should never result in NoChange"
            ),
            MarkTileResult::TileMarked => ControlFlow::Continue(()),
            MarkTileResult::OutcomeDecided(outcome) => ControlFlow::Break(outcome),
        }
    }
    /// Updates the current node to the given `child` and updates the current game state
    /// accordingly.
    ///
    /// The caller must ensure that `child` is a child of `self.current_node`.
    fn visit(&mut self, child: NodeRef) -> ControlFlow<BoardOutcome> {
        let play = child.borrow().play;
        self.current_node = child;
        if let Some(play) = play {
            self.current_player = self.current_player.other();
            self.mark_tile(play)
        } else {
            ControlFlow::Continue(())
        }
    }
    /// Runs one iteration of the MCTS algorithm ending with backpropagating the resulting score up
    /// to the root.
    fn run(&mut self) {
        let outcome = (|| -> ControlFlow<BoardOutcome> {
            let () = self.explore()?;
            let () = self.expand()?;
            ControlFlow::Break(self.rollout())
        })()
        .break_value()
        .expect("closure should only return the break variant");
        self.backpropagate(outcome);
    }
    /// Traverses the tree by selecting the best child at each node until it reaches a leaf node or
    /// a terminal node.
    ///
    /// See also [`Node`] and [`Node::find_best_child`].
    fn explore(&mut self) -> ControlFlow<BoardOutcome> {
        loop {
            let child = self.current_node.borrow().find_best_child();
            if let Some(child) = child {
                let () = self.visit(child)?;
            } else {
                return ControlFlow::Continue(());
            }
        }
    }
    /// Populates the [`Node::children`] field of the current node by enumerating all valid plays
    /// from the current game state, then moves the cursor into one of the new nodes.
    ///
    /// The caller must ensure that the current node is not a *terminal* node, meaning the current
    /// game state must be [`BoardState::InProgress`].
    ///
    /// All possible child nodes are generated at once then shuffled. On future visits, the cursor
    /// can just visit the unvisited children in order and it would still be effectively random.
    fn expand(&mut self) -> ControlFlow<BoardOutcome> {
        assert!(matches!(self.game.state, BoardState::InProgress));

        if self.current_node.borrow().should_rollout() {
            return ControlFlow::Continue(());
        }

        let add_moves_for_region = |(region_index, region): (BoardIndex, &Region)| {
            for (tile_index, _) in region.board.unmarked() {
                let play = (region_index, tile_index);
                Node::add_child(&self.current_node, play);
            }
        };

        if let Some(region_index) = self.game.allowed_region_index() {
            add_moves_for_region((region_index, &self.game.board[region_index]));
        } else {
            self.game.board.unmarked().for_each(add_moves_for_region);
        };
        self.current_node
            .borrow_mut()
            .children
            .shuffle(&mut self.rng);

        let child = Rc::clone(self.current_node.borrow().children.first().expect("an in-progress game should always have at least one possible play, so this node should always have at least one child."));
        self.visit(child)
    }
    /// Runs a simulation of the game from its current state to the end by making random moves,
    /// then returns the outcome.
    fn rollout(&mut self) -> BoardOutcome {
        loop {
            let play = self.rng.generate_move(&self.game);
            if let ControlFlow::Break(outcome) = self.mark_tile(play) {
                return outcome;
            }
        }
    }
    /// Traverses from the current node back up to the root node and updates the score of each node
    /// according to the given outcome.
    fn backpropagate(&mut self, outcome: BoardOutcome) {
        loop {
            // Since the game is played with the players alternating turns, each layer of the tree
            // represents a play by different player. For each node, we assign a score based on
            // whether this is a victory or loss for the player making the move in the current node.
            let score_update = match outcome {
                BoardOutcome::Draw => SCORE_DRAW,
                BoardOutcome::WonBy(winner) => {
                    if winner == self.current_player {
                        SCORE_WIN
                    } else {
                        SCORE_LOSS
                    }
                }
            };
            self.current_node.borrow_mut().update_score(score_update);
            self.current_player = self.current_player.other();
            let parent = if let Some(parent) = &self.current_node.borrow().parent {
                parent
                    .upgrade()
                    .expect("no nodes should be dropped until the search is complete.")
            } else {
                break;
            };
            self.current_node = parent;
        }
        self.game = self.original_game.clone();
        self.current_player = self.game.current_player.other();
    }
}
