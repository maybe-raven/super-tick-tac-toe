#![allow(unused)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    slice,
};

use gloo_console::log;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use yew_agent::oneshot::oneshot;

use common::{
    BoardIndex, BoardItem, BoardOutcome, BoardState, Game, MarkTileResult, Player, Region,
};
use web_time::{Duration, Instant};
use yew::{platform::spawn_local, Callback, UseStateHandle};

use super::random::{self, GenerateMove};

const EXPLORE_PARAM: f32 = 2.0;
const SCORE_SIMULATED_WIN: f32 = 1.0;
const SCORE_SIMULATED_DRAW: f32 = 0.0;
const SCORE_SIMULATED_LOSS: f32 = -1.0;
const SCORE_IMMEDIATE_WIN: f32 = 2.0;
const SCORE_IMMEDIATE_DRAW: f32 = 0.0;
const SCORE_IMMEDIATE_LOSS: f32 = -2.0;

type Play = (BoardIndex, BoardIndex);

#[oneshot]
pub fn MakeMoveTask(mut game: Game) -> Play {
    assert!(matches!(game.state, BoardState::InProgress));

    let timeout = Instant::now() + Duration::from_secs_f32(5.0);

    let mut rng = thread_rng();
    let mut root = Node::new_root();

    loop {
        Node::explore(&root, &mut rng, game.clone());
        if Instant::now() > timeout {
            break;
        }
    }

    let root = root.borrow();
    log!(root.n_visits);
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
        let (region_index, tile_index) = rng.generate_move(&game);
        let result = game.mark_tile(region_index, tile_index);
        match result {
            MarkTileResult::NoChange => unreachable!(
                "generated move should always be valid and should never result in NoChange"
            ),
            MarkTileResult::TileMarked => (),
            MarkTileResult::OutcomeDecided(outcome) => match outcome {
                BoardOutcome::Draw => return SCORE_SIMULATED_DRAW,
                BoardOutcome::WonBy(Player::Cross) => return SCORE_SIMULATED_WIN,
                BoardOutcome::WonBy(Player::Circle) => return SCORE_SIMULATED_LOSS,
            },
        }
    }
}

#[derive(Clone)]
pub(crate) struct Node {
    play: Option<Play>,
    score: f32,
    n_visits: usize,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new_root() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            play: None,
            score: 0.0,
            n_visits: 0,
            children: Vec::new(),
            parent: None,
        }))
    }
    fn add_child(this: &Rc<RefCell<Node>>, play: Play) {
        let node = Self {
            play: Some(play),
            score: 0.0,
            n_visits: 0,
            children: Vec::new(),
            parent: Some(Rc::downgrade(this)),
        };
        this.borrow_mut().children.push(Rc::new(RefCell::new(node)));
    }
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
    fn explore(this: &Rc<RefCell<Node>>, rng: &mut ThreadRng, mut game: Game) -> (f32, usize) {
        assert!(matches!(game.state, BoardState::InProgress));

        let play = this.borrow().play;
        if let Some((region_index, tile_index)) = play {
            let result = game.mark_tile(region_index, tile_index);
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
    fn rollout(&mut self, rng: &mut ThreadRng, mut game: Game) -> f32 {
        assert_eq!(self.n_visits, 0);
        assert_eq!(self.score, 0.0);

        let (region_index, tile_index) = self
            .play
            .expect("all nodes except the root should denote a play from the parent game state");
        let result = game.mark_tile(region_index, tile_index);
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
            + ((parent_n_visits as f32).ln() / self.n_visits as f32).sqrt()
    }
}
