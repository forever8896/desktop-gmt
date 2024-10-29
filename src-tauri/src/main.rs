// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct UserSettings {
    theme: Theme,
    timezone: Option<String>,
    #[serde(rename = "show_seconds")]
    show_seconds: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Theme {
    bg: String,
    text: String,
}

#[tauri::command]
async fn save_settings(
    app_handle: tauri::AppHandle, 
    bg: String, 
    text: String, 
    timezone: Option<String>, 
    show_seconds: bool  // Make sure this matches exactly
) -> Result<(), String> {
    println!("Saving settings: bg={}, text={}, timezone={:?}, show_seconds={}", 
             bg, text, timezone, show_seconds);
    
    let settings = UserSettings {
        theme: Theme { bg, text },
        timezone,
        show_seconds,
    };
    
    let app_dir = app_handle
        .path_resolver()
        .app_config_dir()
        .ok_or("Failed to get app config directory")?;
    
    fs::create_dir_all(&app_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;
    
    let settings_path = app_dir.join("settings.json");
    
    let settings_json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    fs::write(&settings_path, settings_json)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;
    
    Ok(())
}

#[tauri::command]
async fn load_settings(app_handle: tauri::AppHandle) -> Result<UserSettings, String> {
    let app_dir = app_handle
        .path_resolver()
        .app_config_dir()
        .ok_or("Failed to get app config directory")?;
    
    let settings_path = app_dir.join("settings.json");
    println!("Loading settings from: {:?}", settings_path);
    
    if settings_path.exists() {
        let settings_str = fs::read_to_string(&settings_path)
            .map_err(|e| format!("Failed to read settings file: {}", e))?;
        
        match serde_json::from_str(&settings_str) {
            Ok(settings) => Ok(settings),
            Err(_) => {
                if let Ok(partial_settings) = serde_json::from_str::<serde_json::Value>(&settings_str) {
                    let theme = partial_settings.get("theme").and_then(|t| {
                        Some(Theme {
                            bg: t.get("bg")?.as_str()?.to_string(),
                            text: t.get("text")?.as_str()?.to_string(),
                        })
                    });
                    
                    let timezone = partial_settings.get("timezone")
                        .and_then(|t| t.as_str())
                        .map(String::from);
                    
                    Ok(UserSettings {
                        theme: theme.unwrap_or(Theme {
                            bg: "#1E3E62".to_string(),
                            text: "#FF6500".to_string(),
                        }),
                        timezone,
                        show_seconds: true,
                    })
                } else {
                    Err("Failed to parse settings JSON".to_string())
                }
            }
        }
    } else {
        Ok(UserSettings {
            theme: Theme {
                bg: "#1E3E62".to_string(),
                text: "#FF6500".to_string(),
            },
            timezone: None,
            show_seconds: true,
        })
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_settings, load_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
