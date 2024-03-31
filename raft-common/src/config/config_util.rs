use std::io::Read;

use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct RaftConfig {
    pub master: Option<String>,
    pub slaver: Option<Vec<String>>,
    pub raft_log_path: Option<String>,
}

impl RaftConfig {
    pub fn init() -> Self {
        let context = std::fs::File::open("./raft_config.yaml");
        let mut yaml = String::new();
        match context {
            Ok(mut context) => {
                let _ = context.read_to_string(&mut yaml);
            }
            Err(e) => {
                log::error!("load configuration failed: {}", e);
            }
        }

        let conf: RaftConfig = serde_yaml::from_str(yaml.as_str()).unwrap();
        conf
    }
}

#[cfg(test)]
mod test_load {
    use crate::config::config_util::RaftConfig;

    #[test]
    fn test_load_configuration() {
        let _conf = RaftConfig::init();
    }
}
