use frontend::ai::mct::MakeMoveTask;
use yew_agent::Registrable;

fn main() {
    MakeMoveTask::registrar().register();
}
