use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IRBackend {
    LLVM,
    C,
}

impl Default for IRBackend {
    fn default() -> Self {
        IRBackend::LLVM
    }
}

impl std::str::FromStr for IRBackend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "llvm" => Ok(IRBackend::LLVM),
            "c" => Ok(IRBackend::C),
            _ => Err(format!("Unknown backend: {}. Supported backends: llvm, c", s)),
        }
    }
}

impl std::fmt::Display for IRBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRBackend::LLVM => write!(f, "LLVM"),
            IRBackend::C => write!(f, "C"),
        }
    }
}

/// Global backend configuration
static BACKEND_CONFIG: OnceLock<Mutex<IRBackend>> = OnceLock::new();

/// Initialize the backend configuration
pub fn init_backend_config(backend: IRBackend) {
    BACKEND_CONFIG.set(Mutex::new(backend)).unwrap_or_else(|_| {
        panic!("Backend configuration already initialized");
    });
}

/// Initialize backend for tests (allows re-initialization)
pub fn init_backend_config_for_test(backend: IRBackend) {
    if let Some(config) = BACKEND_CONFIG.get() {
        *config.lock().unwrap() = backend;
    } else {
        init_backend_config(backend);
    }
}

/// Get the current backend configuration
pub fn get_current_backend() -> IRBackend {
    BACKEND_CONFIG
        .get()
        .expect("Backend configuration not initialized")
        .lock()
        .unwrap()
        .clone()
}

/// Set the backend configuration (for testing or runtime changes)
pub fn set_backend(backend: IRBackend) {
    if let Some(config) = BACKEND_CONFIG.get() {
        *config.lock().unwrap() = backend;
    } else {
        init_backend_config(backend);
    }
}