use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ScriptSource {
    pub map_id: String,
    pub source: String,
}

/// Metadata for tracking script file modifications (hot-reload support).
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
struct ScriptFileMeta {
    path: std::path::PathBuf,
    modified: std::time::SystemTime,
}

pub struct ScriptLoader {
    scripts: HashMap<String, String>,
    /// Track file paths and modification times for hot-reload (native only).
    #[cfg(not(target_arch = "wasm32"))]
    file_meta: HashMap<String, ScriptFileMeta>,
}

impl ScriptLoader {
    pub fn new() -> Self {
        Self {
            scripts: HashMap::new(),
            #[cfg(not(target_arch = "wasm32"))]
            file_meta: HashMap::new(),
        }
    }

    /// Register a script from a source string (used by tests and embedded scripts).
    pub fn register_script(&mut self, map_id: &str, source: &str) {
        self.scripts.insert(map_id.to_string(), source.to_string());
    }

    pub fn get_script(&self, map_id: &str) -> Option<&str> {
        self.scripts.get(map_id).map(|s| s.as_str())
    }

    pub fn has_script(&self, map_id: &str) -> bool {
        self.scripts.contains_key(map_id)
    }

    pub fn loaded_maps(&self) -> Vec<&str> {
        self.scripts.keys().map(|s| s.as_str()).collect()
    }

    /// Load all `.js` scripts from a directory.
    ///
    /// File names are converted to map IDs by stripping the `.js` extension.
    /// For example, `PalletTown.js` becomes map ID `"PalletTown"`.
    ///
    /// Returns the number of scripts loaded, or an error.
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
        for entry in fs::read_dir(dir)
            .map_err(|e| ScriptLoaderError::IoError(dir.to_string_lossy().to_string(), e))?
        {
            let entry = entry
                .map_err(|e| ScriptLoaderError::IoError(dir.to_string_lossy().to_string(), e))?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) != Some("js") {
                continue;
            }

            let map_id = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| {
                    ScriptLoaderError::InvalidFileName(path.to_string_lossy().to_string())
                })?
                .to_string();

            let source = fs::read_to_string(&path)
                .map_err(|e| ScriptLoaderError::IoError(path.to_string_lossy().to_string(), e))?;

            let modified = fs::metadata(&path)
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

            self.scripts.insert(map_id.clone(), source);
            self.file_meta.insert(
                map_id,
                ScriptFileMeta {
                    path: path.clone(),
                    modified,
                },
            );
            count += 1;
        }

        log::info!("ScriptLoader: loaded {} scripts from {:?}", count, dir);
        Ok(count)
    }

    /// Check for changed files and reload them. Returns the list of map IDs
    /// whose scripts were reloaded.
    ///
    /// Call this periodically in dev mode (e.g., once per second).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn check_reload(&mut self) -> Vec<String> {
        use std::fs;

        let mut reloaded = Vec::new();

        // Collect keys and meta first to avoid borrow conflict
        let entries: Vec<(String, std::path::PathBuf, std::time::SystemTime)> = self
            .file_meta
            .iter()
            .map(|(id, meta)| (id.clone(), meta.path.clone(), meta.modified))
            .collect();

        for (map_id, path, old_modified) in entries {
            let current_modified = match fs::metadata(&path).and_then(|m| m.modified()) {
                Ok(t) => t,
                Err(_) => continue,
            };

            if current_modified > old_modified {
                match fs::read_to_string(&path) {
                    Ok(source) => {
                        self.scripts.insert(map_id.clone(), source);
                        if let Some(meta) = self.file_meta.get_mut(&map_id) {
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

    /// Load embedded scripts compiled into the binary.
    ///
    /// This is used on WASM targets where filesystem access isn't available,
    /// or as a fallback when the scripts directory doesn't exist.
    pub fn load_embedded(&mut self) -> usize {
        let embedded: &[(&str, &str)] = &[
            (
                "PalletTown",
                include_str!("../../../scripts/maps/PalletTown.js"),
            ),
            (
                "RedsHouse1F",
                include_str!("../../../scripts/maps/RedsHouse1F.js"),
            ),
            (
                "RedsHouse2F",
                include_str!("../../../scripts/maps/RedsHouse2F.js"),
            ),
            (
                "BluesHouse",
                include_str!("../../../scripts/maps/BluesHouse.js"),
            ),
            ("OaksLab", include_str!("../../../scripts/maps/OaksLab.js")),
        ];

        let mut count = 0;
        for (map_id, source) in embedded {
            self.scripts.insert(map_id.to_string(), source.to_string());
            count += 1;
        }

        log::info!("ScriptLoader: loaded {} embedded scripts", count);
        count
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
    fn test_load_embedded() {
        let mut loader = ScriptLoader::new();
        let count = loader.load_embedded();
        assert_eq!(count, 5);
        assert!(loader.has_script("PalletTown"));
        assert!(loader.has_script("OaksLab"));
        assert!(loader.has_script("RedsHouse1F"));
        assert!(loader.has_script("RedsHouse2F"));
        assert!(loader.has_script("BluesHouse"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_load_from_directory() {
        let mut loader = ScriptLoader::new();
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("scripts")
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
            .join("..")
            .join("scripts")
            .join("maps");
        if dir.exists() {
            loader.load_from_directory(&dir).unwrap();
            let reloaded = loader.check_reload();
            assert!(reloaded.is_empty(), "No files should have changed");
        }
    }
}
