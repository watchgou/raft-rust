use heed::{
    self,
    types::{OwnedType, Str},
    Database, Env, EnvOpenOptions,
};
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
    // leaderId
    pub fn wirte(&self, leader_id: i32, json: &str) {
        let databases: Database<OwnedType<i32>, Str> = self.env.create_database(None).unwrap();
        let mut wtxn = self.env.write_txn().unwrap();
        let _ = databases.put(&mut wtxn, &leader_id, json);
        wtxn.commit().unwrap();
    }

    pub fn read(&self) {
        let databases: Database<OwnedType<i32>, Str> = self.env.create_database(None).unwrap();
        let rtxn = self.env.read_txn().unwrap();
        for data in databases.iter(&rtxn).unwrap() {
            match data {
                Ok(value) => {
                    println!("{:#?}", value.1);
                }
                Err(e) => {
                    panic!("{:#?}", e);
                }
            }
        }
    }

    pub fn read_last(&self) -> (i32, &str) {
        let databases: Database<OwnedType<i32>, Str> = self.env.create_database(None).unwrap();
        let rtxn = self.env.read_txn().unwrap();
        let data = databases.last(&rtxn).unwrap().unwrap();
        data
    }
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
