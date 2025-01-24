use frontend::app::App;

use tracing::Level;
use tracing_subscriber::{
    filter::Targets,
    fmt::format::{FmtSpan, Pretty},
    prelude::*,
};
use tracing_web::MakeWebConsoleWriter;

fn main() {
    // let filter = Targets::new().with_target("frontend", Level::DEBUG);
    // let fmt_layer = tracing_subscriber::fmt::layer()
    //     .without_time()
    //     .with_thread_ids(true)
    //     .with_ansi(false)
    //     .with_writer(MakeWebConsoleWriter::new())
    //     .with_span_events(FmtSpan::FULL);
    // let perf_layer = tracing_web::performance_layer().with_details_from_fields(Pretty::default());
    //
    // tracing_subscriber::registry()
    //     .with(fmt_layer)
    //     .with(perf_layer)
    //     .with(filter)
    //     .init();

    yew::Renderer::<App>::new().render();
}
