use anyhow::Result;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tokio::fs::File;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub notifications: bool,
    pub debug: bool,
}

// Default settings for debug mode
#[cfg(debug_assertions)]
impl Default for Settings {
    fn default() -> Self {
        Self {
            notifications: true,
            debug: true,
        }
    }
}

// Default settings for --release mode
#[cfg(not(debug_assertions))]
impl Default for Settings {
    fn default() -> Self {
        Self {
            notifications: true,
            debug: false,
        }
    }
}

pub struct SettingsService {
    settings: Mutex<Settings>,
    file_path: String,
}

async fn write_settings(file_path: &str, settings: &Settings) -> Result<()> {
    let file = File::create(file_path).await?;
    serde_json::to_writer(file.into_std().await, settings)?;
    Ok(())
}

impl SettingsService {
    pub async fn new(file_path: &str) -> Result<Self> {
        let file = File::open(file_path).await;
        let settings = if let Ok(file) = file {
            match serde_json::from_reader(file.into_std().await) {
                Ok(settings) => settings,
                Err(err) => {
                    error!("Resetting config file due to parse failure: {}", err);
                    let settings = Settings::default();
                    write_settings(file_path, &settings).await?;
                    settings
                }
            }
        } else {
            let settings = Settings::default();
            write_settings(file_path, &settings).await?;
            settings
        };

        Ok(Self {
            settings: Mutex::new(settings),
            file_path: file_path.to_string(),
        })
    }

    pub async fn get_settings(&self) -> Settings {
        let settings = match self.settings.lock() {
            Ok(settings) => settings,
            Err(err) => {
                error!("Failed to get lock for settings: {}", err);
                return Settings::default();
            }
        };
        settings.clone()
    }

    pub async fn set_settings(&self, settings: Settings) -> Result<Settings> {
        write_settings(&self.file_path, &settings).await?;
        let mut current_settings = self.settings.lock().unwrap();
        *current_settings = settings;
        Ok(current_settings.clone())
    }
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn test_settings() -> anyhow::Result<()> {
        use crate::settings::SettingsService;
        use std::time::SystemTime;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();
        let file_path = format!("/tmp/test_settings_{}.json", timestamp);
        let settings_service = SettingsService::new(&file_path).await?;
        // Verify that the config file was created
        assert!(tokio::fs::metadata(&file_path).await.is_ok());

        let mut settings = settings_service.get_settings().await;
        assert_eq!(settings.notifications, true);
        settings.notifications = false;
        settings_service.set_settings(settings).await?;
        assert!(!settings_service.get_settings().await.notifications);

        // Read it again
        let settings_service = SettingsService::new(&file_path).await?;
        let settings = settings_service.get_settings().await;
        assert!(!settings.notifications);

        // Delete the config file
        tokio::fs::remove_file(file_path).await?;
        Ok(())
    }
}
