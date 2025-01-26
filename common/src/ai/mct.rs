use core::f32;
use std::{
    cell::{Ref, RefCell},
    ops::ControlFlow,
    rc::{Rc, Weak},
};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use tracing::{debug, instrument};

use crate::{BoardIndex, BoardOutcome, BoardState, Game, MarkTileResult, Play, Player, Region};

use super::random::GenerateMove;

const EXPLORE_PARAM: f32 = f32::consts::SQRT_2;
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
/// A *leaf* node is a node with can be expanded (is not *terminal*) but hasn't been expanded yet.
///
/// Usage:
/// Store the root node and navigate the tree with a [`Cursor`].
#[derive(Clone, Debug)]
pub struct Node {
    play: Option<Play>,
    score: f32,
    n_visits: usize,
    children: Vec<NodeRef>,
    parent: Option<Weak<RefCell<Node>>>,
}

impl Node {
    pub fn score(&self) -> f32 {
        self.score
    }
    pub fn n_visits(&self) -> usize {
        self.n_visits
    }
    pub fn children(&self) -> impl Iterator<Item = Ref<Node>> {
        self.children.iter().map(|x| x.borrow())
    }
    fn new_root() -> NodeRef {
        Rc::new(RefCell::new(Self {
            play: None,
            score: 0.0,
            n_visits: 0,
            children: Vec::new(),
            parent: None,
        }))
    }
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
    fn ucb1(&self, lnn: f32) -> f32 {
        if self.n_visits == 0 {
            return f32::INFINITY;
        }
        let ret =
            self.score / self.n_visits as f32 + EXPLORE_PARAM * (lnn / self.n_visits as f32).sqrt();
        debug!(t = self.score, n = self.n_visits, lnn, ret);
        ret
    }
    fn update_score(&mut self, score_update: f32) {
        self.score += score_update;
        self.n_visits += 1;
    }
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
    fn should_rollout(&self) -> bool {
        self.n_visits == 0 && self.play.is_some()
    }
}

struct Cursor {
    node: NodeRef,
    player: Player,
    original_game: Game,
    game: Game,
    rng: ThreadRng,
}

impl Cursor {
    fn new(root_node: NodeRef, game: Game) -> Self {
        Self {
            node: root_node,
            player: game.current_player,
            original_game: game.clone(),
            game,
            rng: thread_rng(),
        }
    }
    fn mark_tile(&mut self, play: Play) -> ControlFlow<BoardOutcome> {
        match self.game.mark_tile(play) {
            MarkTileResult::NoChange => unreachable!(
                "generated move should always be valid and should never result in NoChange"
            ),
            MarkTileResult::TileMarked => ControlFlow::Continue(()),
            MarkTileResult::OutcomeDecided(outcome) => ControlFlow::Break(outcome),
        }
    }
    fn visit(&mut self, child: NodeRef) -> ControlFlow<BoardOutcome> {
        let play = child.borrow().play;
        self.node = child;
        if let Some(play) = play {
            self.player = self.player.other();
            self.mark_tile(play)
        } else {
            ControlFlow::Continue(())
        }
    }
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
    fn explore(&mut self) -> ControlFlow<BoardOutcome> {
        loop {
            let child = self.node.borrow().find_best_child();
            if let Some(child) = child {
                let () = self.visit(child)?;
            } else {
                return ControlFlow::Continue(());
            }
        }
    }
    fn expand(&mut self) -> ControlFlow<BoardOutcome> {
        if self.node.borrow().should_rollout() {
            return ControlFlow::Continue(());
        }

        let add_moves_for_region = |(region_index, region): (BoardIndex, &Region)| {
            for (tile_index, _) in region.board.unmarked() {
                let play = (region_index, tile_index);
                Node::add_child(&self.node, play);
            }
        };

        if let Some(region_index) = self.game.allowed_region_index() {
            add_moves_for_region((region_index, &self.game.board[region_index]));
        } else {
            self.game.board.unmarked().for_each(add_moves_for_region);
        };
        self.node.borrow_mut().children.shuffle(&mut self.rng);

        let child = Rc::clone(self.node.borrow().children.first().expect("an in-progress game should always have at least one possible play, so this node should always have at least one child."));
        self.visit(child)
    }
    fn rollout(&mut self) -> BoardOutcome {
        loop {
            let play = self.rng.generate_move(&self.game);
            if let ControlFlow::Break(outcome) = self.mark_tile(play) {
                return outcome;
            }
        }
    }
    fn backpropagate(&mut self, outcome: BoardOutcome) {
        loop {
            let score_update = match outcome {
                BoardOutcome::Draw => SCORE_DRAW,
                BoardOutcome::WonBy(winner) => {
                    if winner == self.player {
                        SCORE_LOSS
                    } else {
                        SCORE_WIN
                    }
                }
            };
            self.node.borrow_mut().update_score(score_update);
            self.player = self.player.other();
            let parent = if let Some(parent) = &self.node.borrow().parent {
                parent
                    .upgrade()
                    .expect("no nodes should be dropped until the search is complete.")
            } else {
                break;
            };
            self.node = parent;
        }
        self.reset();
    }
    fn reset(&mut self) {
        self.game = self.original_game.clone();
        self.player = self.game.current_player;
    }
}
