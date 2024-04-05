use raft_core;

#[tokio::main]
async fn main() {
    env_logger::init();
    raft_core::start();
}
