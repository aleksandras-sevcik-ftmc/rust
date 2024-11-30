use std::sync::{Arc, Mutex};
use crate::models::cpu_model::CpuInfo;

/// Service responsible for managing CPU information and state
/// Provides thread-safe access to CPU metrics through Arc<Mutex>
/// Implements Clone to allow sharing between different parts of the application
#[derive(Clone)]
pub struct CpuService {
    /// Thread-safe storage for CPU information
    /// Uses Arc for reference counting and Mutex for safe concurrent access
    cpu_info: Arc<Mutex<CpuInfo>>,
}

impl CpuService {
    /// Creates a new instance of CpuService
    /// 
    /// # Returns
    /// * `CpuService` - New service instance with initialized CPU information
    /// 
    /// # Implementation Details
    /// * Creates new CpuInfo instance with current system metrics
    /// * Wraps it in Arc<Mutex> for thread-safe access
    pub fn new() -> Self {
        Self {
            cpu_info: Arc::new(Mutex::new(CpuInfo::new())),
        }
    }

    /// Retrieves current CPU information
    /// 
    /// # Returns
    /// * `CpuInfo` - Updated CPU metrics including usage, cores, frequency, and brand
    /// 
    /// # Implementation Details
    /// * Acquires mutex lock on CPU info
    /// * Updates metrics to current values
    /// * Returns cloned instance to avoid holding lock
    pub async fn get_cpu_info(&self) -> CpuInfo {
        let mut info = self.cpu_info.lock().unwrap();
        info.update();
        info.clone()
    }

    /// Provides access to the underlying thread-safe CPU state
    /// 
    /// # Returns
    /// * `Arc<Mutex<CpuInfo>>` - Cloned reference to the protected CPU information
    /// 
    /// # Usage
    /// * Used when initializing application state
    /// * Allows other components to directly access CPU state when needed
    pub fn get_state(&self) -> Arc<Mutex<CpuInfo>> {
        self.cpu_info.clone()
    }
}

/// Test module for CPU service functionality
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_cpu_service() {
        let service = CpuService::new();
        let info = service.get_cpu_info().await;
        
        assert!(info.usage >= 0.0 && info.usage <= 100.0);
        assert!(info.cores > 0);
        assert!(!info.brand.is_empty());
    }
}