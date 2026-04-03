use std::collections::HashMap;

use crate::MapScriptConfig;

#[derive(Debug, Clone)]
pub struct ScriptSource {
    pub map_id: String,
    pub source: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
struct ScriptFileMeta {
    path: std::path::PathBuf,
    modified: std::time::SystemTime,
}

pub struct ScriptLoader {
    scripts: HashMap<String, String>,
    configs: HashMap<String, MapScriptConfig>,
    #[cfg(not(target_arch = "wasm32"))]
    file_meta: HashMap<String, ScriptFileMeta>,
}

impl ScriptLoader {
    pub fn new() -> Self {
        Self {
            scripts: HashMap::new(),
            configs: HashMap::new(),
            #[cfg(not(target_arch = "wasm32"))]
            file_meta: HashMap::new(),
        }
    }

    pub fn register_script(&mut self, map_id: &str, source: &str) {
        self.scripts.insert(map_id.to_string(), source.to_string());
    }

    pub fn register_config(&mut self, map_id: &str, config: MapScriptConfig) {
        self.configs.insert(map_id.to_string(), config);
    }

    pub fn register_config_json(&mut self, map_id: &str, json: &str) -> Result<(), String> {
        let config: MapScriptConfig = serde_json::from_str(json)
            .map_err(|e| format!("JSON parse error for {}: {}", map_id, e))?;
        self.configs.insert(map_id.to_string(), config);
        Ok(())
    }

    pub fn get_script(&self, map_id: &str) -> Option<&str> {
        self.scripts.get(map_id).map(|s| s.as_str())
    }

    pub fn get_config(&self, map_id: &str) -> Option<&MapScriptConfig> {
        self.configs.get(map_id)
    }

    pub fn has_script(&self, map_id: &str) -> bool {
        self.scripts.contains_key(map_id)
    }

    pub fn has_config(&self, map_id: &str) -> bool {
        self.configs.contains_key(map_id)
    }

    pub fn loaded_maps(&self) -> Vec<&str> {
        self.scripts.keys().map(|s| s.as_str()).collect()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_from_directory(
        &mut self,
        dir: &std::path::Path,
    ) -> Result<usize, ScriptLoaderError> {
        use std::fs;

        if !dir.is_dir() {
            return Err(ScriptLoaderError::NotADirectory(
                dir.to_string_lossy().to_string(),
            ));
        }

        let mut count = 0;
        // Scan subdirectories — each subdirectory is a map name folder
        // containing optional script.js and script_config.json
        for entry in fs::read_dir(dir)
            .map_err(|e| ScriptLoaderError::IoError(dir.to_string_lossy().to_string(), e))?
        {
            let entry = entry
                .map_err(|e| ScriptLoaderError::IoError(dir.to_string_lossy().to_string(), e))?;
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let map_id = path
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| {
                    ScriptLoaderError::InvalidFileName(path.to_string_lossy().to_string())
                })?
                .to_string();

            let js_path = path.join("script.js");
            if js_path.is_file() {
                let content = fs::read_to_string(&js_path).map_err(|e| {
                    ScriptLoaderError::IoError(js_path.to_string_lossy().to_string(), e)
                })?;

                let modified = fs::metadata(&js_path)
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

                self.scripts.insert(map_id.clone(), content);
                self.file_meta.insert(
                    format!("{}:js", map_id),
                    ScriptFileMeta {
                        path: js_path,
                        modified,
                    },
                );
                count += 1;
            }

            let config_path = path.join("script_config.json");
            if config_path.is_file() {
                let content = fs::read_to_string(&config_path).map_err(|e| {
                    ScriptLoaderError::IoError(config_path.to_string_lossy().to_string(), e)
                })?;

                let modified = fs::metadata(&config_path)
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

                let config: MapScriptConfig = serde_json::from_str(&content).map_err(|e| {
                    ScriptLoaderError::IoError(
                        config_path.to_string_lossy().to_string(),
                        std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()),
                    )
                })?;
                self.configs.insert(map_id.clone(), config);
                self.file_meta.insert(
                    format!("{}:json", map_id),
                    ScriptFileMeta {
                        path: config_path,
                        modified,
                    },
                );
                count += 1;
            }
        }

        log::info!("ScriptLoader: loaded {} files from {:?}", count, dir);
        Ok(count)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn check_reload(&mut self) -> Vec<String> {
        use std::fs;

        let mut reloaded = Vec::new();

        let entries: Vec<(String, std::path::PathBuf, std::time::SystemTime)> = self
            .file_meta
            .iter()
            .map(|(id, meta)| (id.clone(), meta.path.clone(), meta.modified))
            .collect();

        for (meta_key, path, old_modified) in entries {
            let current_modified = match fs::metadata(&path).and_then(|m| m.modified()) {
                Ok(t) => t,
                Err(_) => continue,
            };

            if current_modified > old_modified {
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        let ext = path.extension().and_then(|e| e.to_str());
                        let map_id = path
                            .parent()
                            .and_then(|p| p.file_name())
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_string();

                        match ext {
                            Some("js") => {
                                self.scripts.insert(map_id.clone(), content);
                            }
                            Some("json") => {
                                if let Ok(config) =
                                    serde_json::from_str::<MapScriptConfig>(&content)
                                {
                                    self.configs.insert(map_id.clone(), config);
                                }
                            }
                            _ => {}
                        }

                        if let Some(meta) = self.file_meta.get_mut(&meta_key) {
                            meta.modified = current_modified;
                        }
                        log::info!("ScriptLoader: hot-reloaded {:?}", path);
                        reloaded.push(map_id);
                    }
                    Err(e) => {
                        log::warn!("ScriptLoader: failed to reload {:?}: {}", path, e);
                    }
                }
            }
        }

        reloaded
    }

    #[cfg(feature = "embedded-scripts")]
    pub fn load_embedded(&mut self) -> usize {
        let embedded_scripts: &[(&str, &str)] = &[
            (
                "PalletTown",
                include_str!("../../pokered-data/maps/PalletTown/script.js"),
            ),
            (
                "RedsHouse1F",
                include_str!("../../pokered-data/maps/RedsHouse1F/script.js"),
            ),
            (
                "RedsHouse2F",
                include_str!("../../pokered-data/maps/RedsHouse2F/script.js"),
            ),
            (
                "BluesHouse",
                include_str!("../../pokered-data/maps/BluesHouse/script.js"),
            ),
            (
                "OaksLab",
                include_str!("../../pokered-data/maps/OaksLab/script.js"),
            ),
        ];

        let embedded_configs: &[(&str, &str)] = &[
            (
                "PalletTown",
                include_str!("../../pokered-data/maps/PalletTown/script_config.json"),
            ),
            (
                "RedsHouse1F",
                include_str!("../../pokered-data/maps/RedsHouse1F/script_config.json"),
            ),
            (
                "RedsHouse2F",
                include_str!("../../pokered-data/maps/RedsHouse2F/script_config.json"),
            ),
            (
                "BluesHouse",
                include_str!("../../pokered-data/maps/BluesHouse/script_config.json"),
            ),
            (
                "OaksLab",
                include_str!("../../pokered-data/maps/OaksLab/script_config.json"),
            ),
        ];

        let mut count = 0;
        for (map_id, source) in embedded_scripts {
            self.scripts.insert(map_id.to_string(), source.to_string());
            count += 1;
        }

        for (map_id, json) in embedded_configs {
            if let Ok(config) = serde_json::from_str::<MapScriptConfig>(json) {
                self.configs.insert(map_id.to_string(), config);
            } else {
                log::warn!(
                    "ScriptLoader: failed to parse embedded config for {}",
                    map_id
                );
            }
        }

        log::info!("ScriptLoader: loaded {} embedded scripts + configs", count);
        count
    }

    #[cfg(feature = "embedded-scripts")]
    pub fn load_auto(
        &mut self,
        _scripts_dir: Option<&std::path::Path>,
    ) -> Result<usize, ScriptLoaderError> {
        let count = self.load_embedded();
        Ok(count)
    }

    #[cfg(all(not(feature = "embedded-scripts"), not(target_arch = "wasm32")))]
    pub fn load_auto(
        &mut self,
        scripts_dir: Option<&std::path::Path>,
    ) -> Result<usize, ScriptLoaderError> {
        let dir = scripts_dir.ok_or_else(|| {
            ScriptLoaderError::NotADirectory(
                "no scripts directory provided (required without embedded-scripts feature)"
                    .to_string(),
            )
        })?;
        self.load_from_directory(dir)
    }
}

impl Default for ScriptLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum ScriptLoaderError {
    NotADirectory(String),
    IoError(String, std::io::Error),
    InvalidFileName(String),
}

impl std::fmt::Display for ScriptLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotADirectory(path) => write!(f, "Not a directory: {}", path),
            Self::IoError(path, err) => write!(f, "IO error at {}: {}", path, err),
            Self::InvalidFileName(path) => write!(f, "Invalid file name: {}", path),
        }
    }
}

impl std::error::Error for ScriptLoaderError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_get() {
        let mut loader = ScriptLoader::new();
        loader.register_script("TestMap", "function onEnter() {}");
        assert!(loader.has_script("TestMap"));
        assert_eq!(loader.get_script("TestMap"), Some("function onEnter() {}"));
    }

    #[test]
    fn test_register_config_json() {
        let mut loader = ScriptLoader::new();
        let json = r#"{
            "mapScripts": ["stateDefault", "stateOak"],
            "npcs": [{"id": 1, "talk": "talkOak"}],
            "signs": [{"id": 1, "talk": "signLab"}],
            "coordEvents": [{"position": [4, 1], "trigger": "enterRoute1"}]
        }"#;
        loader.register_config_json("TestMap", json).unwrap();
        assert!(loader.has_config("TestMap"));
        let config = loader.get_config("TestMap").unwrap();
        assert_eq!(config.map_scripts.len(), 2);
        assert_eq!(config.npcs.len(), 1);
        assert_eq!(config.npc_talk_fn(1), Some("talkOak"));
        assert_eq!(config.sign_talk_fn(1), Some("signLab"));
        assert_eq!(config.coord_event_fn(4, 1), Some("enterRoute1"));
        assert_eq!(config.resolve_map_script_index("stateOak"), Some(1));
    }

    #[cfg(feature = "embedded-scripts")]
    #[test]
    fn test_load_embedded() {
        let mut loader = ScriptLoader::new();
        let count = loader.load_embedded();
        assert_eq!(count, 5);
        assert!(loader.has_script("PalletTown"));
        assert!(loader.has_script("OaksLab"));
        assert!(loader.has_script("RedsHouse1F"));
        assert!(loader.has_script("RedsHouse2F"));
        assert!(loader.has_script("BluesHouse"));
        assert!(loader.has_config("PalletTown"));
        assert!(loader.has_config("OaksLab"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_load_from_directory() {
        let mut loader = ScriptLoader::new();
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("pokered-data")
            .join("maps");
        if dir.exists() {
            let count = loader.load_from_directory(&dir).unwrap();
            assert!(count >= 5);
            assert!(loader.has_script("PalletTown"));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_check_reload_no_changes() {
        let mut loader = ScriptLoader::new();
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("pokered-data")
            .join("maps");
        if dir.exists() {
            loader.load_from_directory(&dir).unwrap();
            let reloaded = loader.check_reload();
            assert!(reloaded.is_empty(), "No files should have changed");
        }
    }
}
