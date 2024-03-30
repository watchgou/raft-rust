fn main() {
    println!("hello world");
}

#[cfg(test)]
mod test {
    use env_logger;
    use log::info;
    use raft_code::protos;
    use raft_common::raft_state::State;
    #[test]
    fn test() {
        env_logger::init();
        let hello = protos::hello::HotWordRequest::default();
        let value = State::default();
        info!("State: {:#?}", value);
        info!("HotWordRequest: {:?}", hello);
    }
}
