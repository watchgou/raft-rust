use heed::{self, Env, EnvOpenOptions};
use log::info;
use std::{fs, path::Path};

#[derive(Default, Clone, Copy)]
struct LogEntity {}

#[derive(Clone)]
pub struct LogModule {
    env: Env,

    entity: LogEntity,
}

impl LogModule {
    pub fn new(path: Option<&str>) -> Self {
        let path = match path {
            Some(path) => path,
            None => "raft_log",
        };
        fs::create_dir_all(Path::new(path)).unwrap();
        let env = EnvOpenOptions::new().open(Path::new(path)).unwrap();
        info!("raft log path: {:?}", env.path());
        Self {
            env,
            entity: LogEntity::default(),
        }
    }

    pub fn wirte(&self) {}

    pub fn read(&self) {}
}

#[cfg(test)]
mod raft_log_test {

    #[test]
    fn test_log() {
        //env_logger::init();
        match std::env::current_dir() {
            Ok(path) => {
                log::info!("absolute path {:?}", path);
            }
            Err(e) => {
                log::error!("{}", e);
            }
        }
    }
}
