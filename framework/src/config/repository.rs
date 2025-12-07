use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// Global config repository - stores config instances by type
static CONFIG_REPOSITORY: OnceLock<RwLock<ConfigRepository>> = OnceLock::new();

/// Repository for storing typed configuration structs
pub struct ConfigRepository {
    configs: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ConfigRepository {
    /// Create a new empty config repository
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    /// Register a config struct in the repository
    pub fn register<T: Any + Send + Sync + 'static>(&mut self, config: T) {
        self.configs.insert(TypeId::of::<T>(), Box::new(config));
    }

    /// Get a config struct by type
    pub fn get<T: Any + Send + Sync + Clone + 'static>(&self) -> Option<T> {
        self.configs
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
            .cloned()
    }

    /// Check if a config type is registered
    pub fn has<T: Any + 'static>(&self) -> bool {
        self.configs.contains_key(&TypeId::of::<T>())
    }
}

impl Default for ConfigRepository {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize the global config repository
pub fn init_repository() -> &'static RwLock<ConfigRepository> {
    CONFIG_REPOSITORY.get_or_init(|| RwLock::new(ConfigRepository::new()))
}

/// Register a config in the global repository
pub fn register<T: Any + Send + Sync + 'static>(config: T) {
    let repo = init_repository();
    if let Ok(mut repo) = repo.write() {
        repo.register(config);
    }
}

/// Get a config from the global repository
pub fn get<T: Any + Send + Sync + Clone + 'static>() -> Option<T> {
    let repo = CONFIG_REPOSITORY.get()?;
    repo.read().ok()?.get::<T>()
}

/// Check if a config type is registered in the global repository
pub fn has<T: Any + 'static>() -> bool {
    CONFIG_REPOSITORY
        .get()
        .and_then(|repo| repo.read().ok())
        .map(|repo| repo.has::<T>())
        .unwrap_or(false)
}
