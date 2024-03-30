use log::info;
use raft_code::protos;
use raft_common::raft_state::State;
fn main() {
    let hello = protos::hello::HotWordRequest::default();
    let value = State::default();
    info!("State: {:?}", value);
    info!("HotWordRequest: {}", hello);
}
