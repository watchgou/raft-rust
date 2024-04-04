use raft_common::raft_log;

fn main() {
    let module = raft_log::log::LogModule::new(None);
    //module.wirte(2, "xxxx");
    module.read_last();

    // let mut yaml = String::new();
    // let conf: raft_common::RaftConfig = config::config_util::C::parse(
    //     "/Users/jon/workspace/rust/raft-rust/raft_config.yaml",
    //     &mut yaml,
    // );
    // println!("hello world {:?}, {:?}", conf, yaml);
}
