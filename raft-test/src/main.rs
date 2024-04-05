use raft_core;

#[tokio::main]
async fn main() {
    env_logger::init();
    let _ = raft_core::start().await;
}
