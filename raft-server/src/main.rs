use raft_common::config;
use raft_common::config::config_util::ParseConfig;
use raft_common::raft_log;

fn main() {
    let _module = raft_log::log::LogModule::new(None);
    let mut yaml = String::new();
    let conf: raft_common::RaftConfig = config::config_util::C::parse(
        "/Users/jon/workspace/rust/raft-rust/raft_config.yaml",
        &mut yaml,
    );
    println!("hello world {:?}, {:?}", conf, yaml);
}
