use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, System, SystemExt};

/// Represents comprehensive CPU information including usage, cores, frequency, and brand
/// Implements serialization/deserialization for API communication
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpuInfo {
    /// Current CPU usage as a percentage (0.0 to 100.0)
    pub usage: f32,
    
    /// Number of CPU cores available in the system
    pub cores: usize,
    
    /// Current CPU frequency in MHz
    pub frequency: u64,
    
    /// CPU brand/model information as string
    pub brand: String,
}

impl CpuInfo {
    /// Creates a new CpuInfo instance with current system information
    /// 
    /// # Returns
    /// * `CpuInfo` - New instance populated with current CPU metrics
    /// 
    /// # Implementation Details
    /// * Creates new System instance for hardware monitoring
    /// * Calculates average CPU usage across all cores
    /// * Retrieves core count, frequency, and brand information
    pub fn new() -> Self {
        // Initialize system information gatherer
        let mut sys = System::new_all();
        sys.refresh_all();
        
        // Calculate average CPU usage across all cores
        let usage = sys.cpus().iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>() / sys.cpus().len() as f32;
            
        // Get total number of CPU cores
        let cores = sys.cpus().len();
        
        // Get CPU frequency from first core (usually representative)
        // Defaults to 0 if information unavailable
        let frequency = sys.cpus().first()
            .map(|cpu| cpu.frequency())
            .unwrap_or(0);
            
        // Get CPU brand information from first core
        // Defaults to "Unknown" if information unavailable
        let brand = sys.cpus().first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| String::from("Unknown"));

        Self {
            usage,
            cores,
            frequency,
            brand,
        }
    }

    /// Updates the CPU usage information in the existing instance
    /// 
    /// # Implementation Details
    /// * Refreshes system information
    /// * Recalculates average CPU usage across all cores
    /// * Updates only usage field, leaving other fields unchanged
    pub fn update(&mut self) {
        // Reinitialize system information gatherer
        let mut sys = System::new_all();
        sys.refresh_all();
        
        // Recalculate average CPU usage
        self.usage = sys.cpus().iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>() / sys.cpus().len() as f32;
    }
}