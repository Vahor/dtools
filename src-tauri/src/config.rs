use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};
use thiserror::Error;
use tracing::info;

use crate::sniffer::config::NetworkConfig;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfig {
    pub network: NetworkConfig,
    pub game_version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub version: String,
    pub check_for_updates: bool,
}

struct ConfigLoader<ConfigType> {
    _config_type: std::marker::PhantomData<ConfigType>,
}

impl<ConfigType> ConfigLoader<ConfigType>
where
    ConfigType: for<'de> Deserialize<'de> + for<'a> Serialize + specta::Type,
{
    pub async fn load(path: impl AsRef<Path>) -> Result<ConfigType, ConfigError> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        let config: ConfigType = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub async fn save(config: &ConfigType, path: impl AsRef<Path>) -> Result<(), ConfigError> {
        ConfigLoader::save_sync(config, path)
    }

    pub fn save_sync(config: &ConfigType, path: impl AsRef<Path>) -> Result<(), ConfigError> {
        let config = serde_json::to_string_pretty(config)?;
        std::fs::write(path, config)?;
        Ok(())
    }
}

impl Default for NodeConfig {
    fn default() -> Self {
        NodeConfig {
            network: NetworkConfig::default(),
            game_version: Version {
                version: "".to_string(),
                check_for_updates: false,
            },
        }
    }
}

#[derive(Debug)]
pub struct Manager<ConfigType> {
    pub config: RwLock<ConfigType>,
    pub data_dir_path: PathBuf,
    pub config_file_path: PathBuf,
}

impl<ConfigType> Manager<ConfigType>
where
    ConfigType: for<'de> Deserialize<'de> + Default + for<'a> Serialize + specta::Type,
{
    pub async fn new(
        data_dir: impl AsRef<Path>,
        file_name: &'static str,
    ) -> Result<Arc<Self>, ConfigError> {
        let data_dir = data_dir.as_ref();
        let data_dir_path = data_dir.to_path_buf();
        let config_file_path = data_dir.join(file_name);

        let config = if config_file_path.exists() {
            ConfigLoader::<ConfigType>::load(&config_file_path).await?
        } else {
            let default = ConfigType::default();
            ConfigLoader::save(&default, &config_file_path).await?;
            default
        };

        let config = RwLock::new(config);

        info!("Config loaded successfully from {:?}", config_file_path);
        Ok(Arc::new(Manager {
            config,
            data_dir_path,
            config_file_path,
        }))
    }

    pub async fn update_config(
        &self,
        update_fn: impl FnOnce(&mut ConfigType),
    ) -> Result<(), ConfigError> {
        let mut config = self.config.write().unwrap();
        update_fn(&mut config);
        ConfigLoader::<ConfigType>::save(&config, &self.config_file_path)
            .await
            .map_err(Into::into)
    }

    pub fn update_config_sync(
        &self,
        update_fn: impl FnOnce(&mut ConfigType),
    ) -> Result<(), ConfigError> {
        let mut config = self.config.write().unwrap();
        update_fn(&mut config);
        ConfigLoader::<ConfigType>::save_sync(&config, &self.config_file_path).map_err(Into::into)
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    FileIo(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
