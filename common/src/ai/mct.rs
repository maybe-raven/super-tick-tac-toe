use std::{
    cell::{Ref, RefCell},
    ops::ControlFlow,
    rc::{Rc, Weak},
};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use tracing::{debug, instrument};

use crate::{
    BoardIndex, BoardItem, BoardOutcome, BoardState, Game, MarkTileResult, Play, Player, Region,
};

use super::random::GenerateMove;

const EXPLORE_PARAM: f32 = 2.0;
const SCORE_WIN: f32 = 1.0;
const SCORE_DRAW: f32 = 0.0;
const SCORE_LOSS: f32 = -1.0;

#[instrument(skip(should_terminate, game))]
pub fn make_move(game: Game, should_terminate: impl Fn(&Node) -> bool) -> Play {
    assert!(matches!(game.state, BoardState::InProgress));

    let mut rng = thread_rng();
    let root = Node::new_root();

    loop {
        Node::run(root.clone(), &mut rng, game.clone());
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

fn score_result(result: MarkTileResult) -> ControlFlow<f32, ()> {
    match result {
        MarkTileResult::NoChange => unreachable!(
            "generated move should always be valid and should never result in NoChange"
        ),
        MarkTileResult::TileMarked => ControlFlow::Continue(()),
        MarkTileResult::OutcomeDecided(outcome) => {
            let score = match outcome {
                BoardOutcome::Draw => SCORE_DRAW,
                BoardOutcome::WonBy(Player::Cross) => SCORE_WIN,
                BoardOutcome::WonBy(Player::Circle) => SCORE_LOSS,
            };
            ControlFlow::Break(score)
        }
    }
}

fn simulate(mut game: Game, rng: &mut ThreadRng) -> f32 {
    loop {
        let play = rng.generate_move(&game);
        let result = game.mark_tile(play);
        if let ControlFlow::Break(score) = score_result(result) {
            return score;
        }
    }
}

type NodeRef = Rc<RefCell<Node>>;

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
    #[instrument(skip(this, rng, game), fields(t=this.borrow().score, n=this.borrow().n_visits, n_children=this.borrow().children.len()))]
    fn expand(this: &mut NodeRef, rng: &mut ThreadRng, game: &mut Game) -> ControlFlow<f32, ()> {
        assert!(matches!(game.state, BoardState::InProgress));
        assert! {
            if let Some((region_index, tile_index)) = this.borrow().play {
                !game.board[region_index].board[tile_index].is_markable()
            } else {
                true
            }
        };

        let add_moves_for_region = |(region_index, region): (BoardIndex, &Region)| {
            for (tile_index, _) in region.board.unmarked() {
                let play = (region_index, tile_index);
                Node::add_child(&*this, play);
            }
        };

        if let Some(region_index) = game.allowed_region_index() {
            add_moves_for_region((region_index, &game.board[region_index]));
        } else {
            game.board.unmarked().for_each(add_moves_for_region);
        };

        debug!(n_children = this.borrow().children.len());

        this.borrow_mut().children.shuffle(rng);
        let child = Rc::clone(this.borrow().children.first().expect("an in-progress game should always have at least one possible play, so this node should always have at least one child."));
        let result =
            game.mark_tile(child.borrow().play.expect(
                "all nodes except the root should denote a play from the parent game state",
            ));
        let () = score_result(result)?;
        *this = child;
        ControlFlow::Continue(())
    }
    #[instrument(skip(this, _rng, game), fields(t=this.borrow().score, n=this.borrow().n_visits, n_children=this.borrow().children.len()))]
    fn explore(this: &mut NodeRef, _rng: &mut ThreadRng, game: &mut Game) -> ControlFlow<f32, ()> {
        assert!(matches!(game.state, BoardState::InProgress));

        let mut invert_score = false;

        loop {
            let play = this.borrow().play;
            if let Some(play) = play {
                let result = game.mark_tile(play);
                let () = score_result(result)?;
            }

            let lnn = (this.borrow().n_visits as f32).ln();
            let child = this
                .borrow()
                .children
                .iter()
                .max_by(|node0: &&NodeRef, node1: &&NodeRef| {
                    node0
                        .borrow()
                        .ucb1(lnn, invert_score)
                        .total_cmp(&node1.borrow().ucb1(lnn, invert_score))
                })
                .map(Rc::clone);

            if let Some(child) = child {
                *this = child;
            } else {
                return ControlFlow::Continue(());
            }

            invert_score = !invert_score;
        }
    }
    #[instrument(skip(self, rng, game), fields(t=self.score, n=self.n_visits, n_children=self.children.len()))]
    fn rollout(&mut self, rng: &mut ThreadRng, game: Game) -> f32 {
        assert_eq!(self.n_visits, 0);
        assert_eq!(self.score, 0.0);

        let score = simulate(game, rng);
        debug!(score);
        score
    }
    fn ucb1(&self, lnn: f32, invert_score: bool) -> f32 {
        if self.n_visits == 0 {
            return f32::INFINITY;
        }
        let ret = if invert_score { -1.0 } else { 1.0 } * self.score / self.n_visits as f32
            + EXPLORE_PARAM * (lnn / self.n_visits as f32).sqrt();
        debug!(t = self.score, n = self.n_visits, lnn, ret);
        ret
    }
    #[instrument(skip(this), fields(t=this.borrow().score, n=this.borrow().n_visits, n_children=this.borrow().children.len()))]
    fn backpropagate(mut this: NodeRef, score_update: f32) {
        loop {
            this = {
                let mut this = this.borrow_mut();
                this.score += score_update;
                this.n_visits += 1;
                if let Some(parent) = &this.parent {
                    parent
                        .upgrade()
                        .expect("no nodes should be dropped until the search is complete.")
                } else {
                    return;
                }
            };
        }
    }
    #[instrument(skip(this,rng, game), fields(t=this.borrow().score, n=this.borrow().n_visits, n_children=this.borrow().children.len()))]
    fn run(mut this: NodeRef, rng: &mut ThreadRng, mut game: Game) {
        let score_update = || -> ControlFlow<f32, ()> {
            let () = Self::explore(&mut this, rng, &mut game)?;
            if this.borrow().play.is_none() || this.borrow().n_visits > 0 {
                let () = Self::expand(&mut this, rng, &mut game)?;
            }
            ControlFlow::Break(this.borrow_mut().rollout(rng, game))
        }()
        .break_value()
        .expect("should only be break at this point.");
        Self::backpropagate(this, score_update);
    }
}
