use heed::{
    self,
    types::{OwnedType, Str},
    Database, Env, EnvOpenOptions,
};
use log::info;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LogEntity {
    pub index: u32,
    pub term: u32,
    pub command: Option<Command>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Command {
    pub key: String,
    pub value: String,
}

#[derive(Clone)]
pub struct LogModule {
    env: Env,
}

type MDBase = Database<OwnedType<u32>, Str>;

impl LogModule {
    pub fn new(path: Option<&str>) -> Self {
        let path = match path {
            Some(path) => path,
            None => "raft_log",
        };
        fs::create_dir_all(Path::new(path)).unwrap();
        let env = EnvOpenOptions::new().open(Path::new(path)).unwrap();
        info!("raft log path: {:?}", env.path());
        Self { env }
    }
    // leaderId
    pub fn wirte(&self, entity: LogEntity) {
        if let Some(database) = self.data_base() {
            let mut wtxn = self.env.write_txn().unwrap();
            let log = serde_json::to_string(&entity).unwrap();

            let _ = database.put(&mut wtxn, &entity.index, log.as_str());
            wtxn.commit().unwrap();
        }
    }

    pub fn read_last(&self) -> (u32, LogEntity) {
        if let Some(database) = self.data_base() {
            let rtxn = self.env.read_txn().unwrap();
            let data = database.last(&rtxn).unwrap().unwrap();
            (data.0, serde_json::from_str(data.1).unwrap())
        } else {
            (0, LogEntity::default())
        }
    }

    pub fn exist_index(&self, index: u32) -> bool {
        if let Some(database) = self.data_base() {
            let rtxn = self.env.read_txn().unwrap();
            match database.get(&rtxn, &index) {
                Ok(value) => {
                    if let Some(_) = value {
                        true
                    } else {
                        false
                    }
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }

    pub fn index_size(&self) -> u64 {
        if let Some(database) = self.data_base() {
            let rtxn = self.env.read_txn().unwrap();
            let len = database.len(&rtxn).unwrap();
            len
        } else {
            0
        }
    }

    pub fn read(&self) {
        if let Some(database) = self.data_base() {
            let rtxn = self.env.read_txn().unwrap();
            for data in database.iter(&rtxn).unwrap() {
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
    }

    pub fn delete(&self, index: u32) {
        if let Some(database) = self.data_base() {
            let mut wtxn = self.env.write_txn().unwrap();
            let _ = database.delete(&mut wtxn, &index);
            wtxn.commit().unwrap();
        }
    }

    fn data_base(&self) -> Option<MDBase> {
        let database: Option<MDBase> = match self.env.create_database(None) {
            Ok(database) => Some(database),
            Err(e) => {
                log::error!("failed create database :{:?}", e);
                None
            }
        };
        database
    }
}
