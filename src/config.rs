macro_rules! config {
    ($x:ident) => {
        ::CONFIG.$x()
    };
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_verbosity")]
    verbosity: String,
    api_key: String,
    #[serde(default = "default_data_dir")]
    data_dir: String,
    #[serde(default = "default_save_interval")]
    save_interval: u64,
    #[serde(default = "default_command_prefix")]
    command_prefix: String,
}

fn default_verbosity() -> String {
    "Info".to_string()
}
fn default_data_dir() -> String {
    "data/".to_string()
}
fn default_save_interval() -> u64 {
    1800
}
fn default_command_prefix() -> String {
    "!".to_string()
}

impl Config {
    pub fn verbosity(&self) -> &str {
        &self.verbosity
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn data_dir(&self) -> &str {
        &self.data_dir
    }

    pub fn save_interval(&self) -> &u64 {
        &self.save_interval
    }

    pub fn command_prefix(&self) -> &str {
        &self.command_prefix
    }
}
