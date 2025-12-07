/// Configuration for Inertia.js integration
pub struct InertiaConfig {
    /// Vite dev server URL (e.g., "http://localhost:5173")
    pub vite_dev_server: String,
    /// Entry point for the frontend (e.g., "src/main.tsx")
    pub entry_point: String,
    /// Asset version for cache busting
    pub version: String,
    /// Whether we're in development mode (use Vite dev server)
    pub development: bool,
}

impl Default for InertiaConfig {
    fn default() -> Self {
        Self {
            vite_dev_server: "http://localhost:5173".to_string(),
            entry_point: "src/main.tsx".to_string(),
            version: "1.0".to_string(),
            development: true,
        }
    }
}

impl InertiaConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vite_dev_server(mut self, url: impl Into<String>) -> Self {
        self.vite_dev_server = url.into();
        self
    }

    pub fn entry_point(mut self, entry: impl Into<String>) -> Self {
        self.entry_point = entry.into();
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn production(mut self) -> Self {
        self.development = false;
        self
    }
}
