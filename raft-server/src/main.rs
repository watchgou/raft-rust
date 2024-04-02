use raft_common::raft_log;

fn main() {
    let _module = raft_log::log::LogModule::new(None);

    println!("hello world");
}

#[cfg(test)]
mod test {
    use env_logger;
    use log::info;
    use raft_code::protos;
    #[test]
    fn test() {
        env_logger::init();
        let hello = protos::hello::HotWordRequest::default();
        info!("HotWordRequest: {:?}", hello);
    }
}
