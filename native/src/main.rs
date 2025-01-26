// use tracing_subscriber::{filter::LevelFilter, fmt::format::FmtSpan, prelude::*, Registry};

use std::time::{Duration, Instant};

use common::{
    ai::{self},
    BoardIndex, Game,
};

fn main() {
    // let stdout_log = tracing_subscriber::fmt::layer()
    //     .with_ansi(false)
    //     .with_span_events(FmtSpan::ACTIVE)
    //     .pretty()
    //     .with_filter(LevelFilter::DEBUG);
    // Registry::default().with(stdout_log).init();
    // let mut game = Game::new();
    // let play = ai::mct::make_move(game.clone(), |node| node.n_visits() > 200);
    // game.mark_tile(play);

    let mut total_time = Duration::ZERO;
    let mut game = Game::new();
    game.mark_tile((BoardIndex::Center, BoardIndex::Up));
    let mut games = vec![game; 10];

    for game in games.iter_mut() {
        let start_time = Instant::now();
        let play = ai::mct::make_move(game.clone(), |node| {
            if node.n_visits() > 20000 {
                println!("score average: {}", node.average_score());
                println!(
                    "{:?}",
                    node.children()
                        .map(|child| child.average_score())
                        .collect::<Vec<_>>()
                );
                true
            } else {
                false
            }
        });
        let end_time = Instant::now();
        let diff = end_time - start_time;
        total_time += diff;
        println!("{:?}", diff);
        game.mark_tile(play);
    }

    println!("average runtime: {:?}", total_time / 10);
    // println!("{:?}", games[0].state);
}
