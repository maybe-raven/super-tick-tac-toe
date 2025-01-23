use std::{cell::RefCell, rc::Rc};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use tracing::{debug, instrument};

use crate::{
    BoardIndex, BoardItem, BoardOutcome, BoardState, Game, MarkTileResult, Play, Player, Region,
};

use super::random::GenerateMove;

const EXPLORE_PARAM: f32 = 2.0;
const SCORE_SIMULATED_WIN: f32 = 1.0;
const SCORE_SIMULATED_DRAW: f32 = 0.0;
const SCORE_SIMULATED_LOSS: f32 = -1.0;
const SCORE_IMMEDIATE_WIN: f32 = 2.0;
const SCORE_IMMEDIATE_DRAW: f32 = 0.0;
const SCORE_IMMEDIATE_LOSS: f32 = -2.0;

#[instrument(skip(should_terminate))]
pub fn make_move(game: Game, should_terminate: impl Fn() -> bool) -> Play {
    assert!(matches!(game.state, BoardState::InProgress));

    let mut rng = thread_rng();
    let root = Node::new_root();

    loop {
        Node::explore(&root, &mut rng, game.clone());
        if should_terminate() {
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

fn simulate(mut game: Game, rng: &mut ThreadRng) -> f32 {
    loop {
        let play = rng.generate_move(&game);
        let result = game.mark_tile(play);
        match result {
            MarkTileResult::NoChange => unreachable!(
                "generated move should always be valid and should never result in NoChange"
            ),
            MarkTileResult::TileMarked => (),
            MarkTileResult::OutcomeDecided(outcome) => {
                let score = match outcome {
                    BoardOutcome::Draw => SCORE_SIMULATED_DRAW,
                    BoardOutcome::WonBy(Player::Cross) => SCORE_SIMULATED_WIN,
                    BoardOutcome::WonBy(Player::Circle) => SCORE_SIMULATED_LOSS,
                };
                debug!("simulated win");
                return score;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Node {
    play: Option<Play>,
    score: f32,
    n_visits: usize,
    children: Vec<Rc<RefCell<Node>>>,
    // parent: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new_root() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            play: None,
            score: 0.0,
            n_visits: 0,
            children: Vec::new(),
            // parent: None,
        }))
    }
    fn add_child(this: &Rc<RefCell<Node>>, play: Play) {
        let node = Self {
            play: Some(play),
            score: 0.0,
            n_visits: 0,
            children: Vec::new(),
            // parent: Some(Rc::downgrade(this)),
        };
        this.borrow_mut().children.push(Rc::new(RefCell::new(node)));
    }
    #[instrument]
    fn expand(this: &Rc<RefCell<Node>>, rng: &mut ThreadRng, game: Game) -> (f32, usize) {
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
                Node::add_child(this, (region_index, tile_index));
            }
        };

        if let Some(region_index) = game.allowed_region_index() {
            add_moves_for_region((region_index, &game.board[region_index]));
        } else {
            game.board.unmarked().for_each(add_moves_for_region);
        };

        this.borrow_mut().children.shuffle(rng);
        let mut total_score = 0.0;
        for child in this.borrow().children.iter() {
            total_score += child.borrow_mut().rollout(rng, game.clone());
        }
        let n = this.borrow().children.len();
        this.borrow_mut().n_visits += n;
        this.borrow_mut().score += total_score;
        (total_score, n)
    }
    #[instrument]
    fn explore(this: &Rc<RefCell<Node>>, rng: &mut ThreadRng, mut game: Game) -> (f32, usize) {
        assert!(matches!(game.state, BoardState::InProgress));

        let play = this.borrow().play;
        if let Some(play) = play {
            let result = game.mark_tile(play);
            match result {
                MarkTileResult::NoChange => panic!(
                "only markable indices should be used and this should never results in NoChange."
            ),
                MarkTileResult::TileMarked => (),
                MarkTileResult::OutcomeDecided(outcome) => {
                    let mut this = this.borrow_mut();
                    let additional_score = match outcome {
                        BoardOutcome::Draw => SCORE_IMMEDIATE_DRAW,
                        BoardOutcome::WonBy(Player::Cross) => SCORE_IMMEDIATE_WIN,
                        BoardOutcome::WonBy(Player::Circle) => SCORE_IMMEDIATE_LOSS,
                    };
                    this.score += additional_score;
                    this.n_visits += 1;
                    debug!("immediate win");
                    return (additional_score, 1);
                }
            }
        }

        let n_visits = this.borrow().n_visits;
        let best_child = this
            .borrow()
            .children
            .iter()
            .max_by(|node0, node1| {
                node0
                    .borrow()
                    .ucb1(n_visits)
                    .total_cmp(&node1.borrow().ucb1(n_visits))
            })
            .map(Rc::clone);

        let (additional_score, n) = if let Some(best_child) = best_child {
            Self::explore(&best_child, rng, game)
        } else {
            Self::expand(this, rng, game)
        };
        this.borrow_mut().score += additional_score;
        this.borrow_mut().n_visits += n;

        (additional_score, n)
    }
    #[instrument]
    fn rollout(&mut self, rng: &mut ThreadRng, mut game: Game) -> f32 {
        assert_eq!(self.n_visits, 0);
        assert_eq!(self.score, 0.0);

        let play = self
            .play
            .expect("all nodes except the root should denote a play from the parent game state");
        let result = game.mark_tile(play);
        match result {
            MarkTileResult::NoChange => panic!(
                "only markable indices should be used and this should never results in NoChange."
            ),
            MarkTileResult::TileMarked => self.score = simulate(game, rng),
            MarkTileResult::OutcomeDecided(outcome) => {
                self.score = match outcome {
                    BoardOutcome::Draw => SCORE_IMMEDIATE_DRAW,
                    BoardOutcome::WonBy(Player::Cross) => SCORE_IMMEDIATE_WIN,
                    BoardOutcome::WonBy(Player::Circle) => SCORE_IMMEDIATE_LOSS,
                }
            }
        }
        self.n_visits = 1;
        self.score
    }
    fn ucb1(&self, parent_n_visits: usize) -> f32 {
        self.score / self.n_visits as f32
            + EXPLORE_PARAM * ((parent_n_visits as f32).ln() / self.n_visits as f32).sqrt()
    }
}
