fn main() {}

#[cfg(test)]
mod raft_log_test {
    use raft_common::{
        config::config_util::{ParseConfig, C},
        raft_log::{
            self,
            log::{Command, LogEntity},
        },
    };
    use serde::Deserialize;
    #[derive(Default, Deserialize, Debug)]
    struct RaftConfig {
        cluster: Option<Vec<String>>,
        raft_log_path: Option<String>,
    }

    #[test]
    fn test_log() {
        env_logger::init();
        let module = raft_log::log::LogModule::new(None);
        let logs = LogEntity {
            index: 1,
            term: 1,
            command: Some(Command::default()),
        };
        module.wirte(logs);
        let result = module.read_last();
        log::info!("{:#?}  {:#?}", result.0, result.1);
        let size = module.index_size();
        log::info!("{:#?}  ", size);
        module.delete(1);
        module.read();

        let mut yaml = String::new();
        let conf: RaftConfig = C::parse(
            "/Users/jon/workspace/rust/raft-rust/raft_config.yaml",
            &mut yaml,
        );
        log::info!("hello world {:?}, {:?}", conf, yaml);
    }
}
