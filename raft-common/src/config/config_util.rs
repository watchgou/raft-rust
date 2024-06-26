use std::io::Read;

use serde::Deserialize;

pub struct C;

pub trait ParseConfig<'d, T> {
    fn parse(path: &str, yaml: &'d mut String) -> T
    where
        T: Deserialize<'d> + Default;
}

impl<'d, T> ParseConfig<'d, T> for C {
    fn parse(path: &str, yaml: &'d mut String) -> T
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
