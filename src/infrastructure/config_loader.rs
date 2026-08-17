use config::{Config, Environment, File, FileFormat};
use serde::de::DeserializeOwned;
use std::env;
use std::fmt::Debug;
use std::path::PathBuf;
use tracing::{error, info};

static DEFAULT_YAML: &str = include_str!("../../config.yaml");

pub fn load<C>(conf_path: &Option<String>) -> C
where
    C: DeserializeOwned + Debug,
{
    let conf_paths = get_config_paths(conf_path);

    let config = conf_paths
        .iter()
        .map(|(path, required)| File::from(path.to_path_buf()).required(required.clone()))
        .fold(
            Config::builder().add_source(File::from_str(DEFAULT_YAML, FileFormat::Yaml)),
            |builder, file| builder.add_source(file),
        )
        .add_source(Environment::with_prefix("ORY_UI").separator("__"))
        .build()
        .inspect_err(|error| error!("Config Load -> FAILED: error=({})", error))
        .unwrap()
        .try_deserialize()
        .inspect_err(|error| error!("Config Deserialize -> FAILED: error=({})", error))
        .unwrap();

    info!(
        "Config Load -> SUCCESS: [path, required]=({:?}), config=({:?})",
        conf_paths, config
    );

    config
}

fn get_config_paths(conf_path: &Option<String>) -> Vec<(PathBuf, bool)> {
    let env = env::var("APP_ENV").unwrap_or_else(|_| "dev".into());
    let cwd = env::current_dir()
        .inspect_err(|error| error!("Get Current Working Directory -> FAILED. error=({})", error))
        .unwrap();

    match conf_path {
        None => vec![
            (cwd.join("config.yml"), false),
            (cwd.join(format!("config-{}.yml", env)), false),
            (cwd.join("/etc/flow-rclone/config.yml"), false),
        ],
        Some(path) => vec![(cwd.join(path), true)],
    }
}
