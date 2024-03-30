use heed::{self, Env, EnvOpenOptions};
use std::{env, fs, path::Path};

#[derive(Default, Clone, Copy)]
struct LogEntity {}

#[derive(Clone)]
pub struct LogModule {
    env: Env,

    entity: LogEntity,
}

impl LogModule {
    pub fn new(path: &str) -> Self {
        let path = if path.is_empty() { "raft_log" } else { path };

        fs::create_dir_all(Path::new(path)).unwrap();

        let env = EnvOpenOptions::new().open(Path::new(path)).unwrap();

        Self {
            env: env,
            entity: LogEntity::default(),
        }
    }

    pub fn wirte(&self) {}

    pub fn read(&self) {}
}

#[cfg(test)]
mod raft_log_test {
    use super::*;

    #[test]
    fn test_log() {
        env_logger::init();
        match env::current_dir() {
            Ok(path) => {
                log::info!("absolute path {:?}", path);
            }
            Err(e) => {
                log::error!("{}", e);
            }
        }
    }
}
