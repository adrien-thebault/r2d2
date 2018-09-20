mod admin;
mod misc;
mod pendu;
mod quiz;
mod slack_client;
mod stats;

pub use self::admin::*;
pub use self::misc::*;
pub use self::pendu::*;
pub use self::quiz::*;
pub use self::slack_client::*;
pub use self::stats::*;

use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs::File, io::Write};

fn load<T: Default>(path: &str) -> T
where
    for<'de> T: Deserialize<'de>,
{
    match File::open(path) {
        Ok(file) => match serde_json::from_reader::<File, T>(file) {
            Ok(res) => res,
            Err(e) => {
                warn!("could not deserialize {}", path);
                warn!("{}", e);
                T::default()
            }
        },
        Err(e) => {
            warn!("could not deserialize {}", path);
            warn!("{}", e);
            T::default()
        }
    }
}

fn save<T: Serialize>(path: &str, data: T) {
    match File::create(path) {
        Ok(mut file) => match serde_json::to_string(&data) {
            Ok(data) => match file.write_all(&data.into_bytes()) {
                Ok(_) => info!("{} : data saved", path),
                Err(e) => {
                    warn!("{} : could not write data", path);
                    warn!("{}", e);
                }
            },
            Err(e) => {
                warn!("couldn't serialize data");
                warn!("{}", e);
            }
        },
        Err(e) => {
            warn!("couldn't open {}", path);
            warn!("{}", e);
        }
    }
}
