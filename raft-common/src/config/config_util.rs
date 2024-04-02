use std::io::Read;

use serde::Deserialize;

pub struct C;

pub trait ParseConfig<'d, T> {
    fn read(path: &str, yaml: &'d mut String) -> T
    where
        T: Deserialize<'d> + Default;
}

impl<'d, T> ParseConfig<'d, T> for C {
    fn read(path: &str, yaml: &'d mut String) -> T
    where
        T: Deserialize<'d> + Default,
    {
        let context = std::fs::File::open(path);
        match context {
            Ok(mut context) => {
                let _ = context.read_to_string(yaml);
            }
            Err(e) => {
                log::error!("load configuration failed: {}", e);
            }
        }

        let t: T = serde_yaml::from_str(yaml.as_str()).unwrap();
        t
    }
}

#[derive(Default, Deserialize, Debug)]
pub struct RaftConfig {
    pub master: Option<String>,
    pub slaver: Option<Vec<String>>,
    pub raft_log_path: Option<String>,
}

#[cfg(test)]
mod test_load {
    use super::ParseConfig;
    use crate::config::config_util::RaftConfig;
    use crate::config::config_util::C;

    #[test]
    fn test_load_configuration() {
        let mut yaml = String::new();
        let read: RaftConfig = C::read("path", &mut yaml);
    }
}
