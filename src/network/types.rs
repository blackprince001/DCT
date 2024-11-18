use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Copy)]
pub enum DataSize {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
}

impl DataSize {
    pub fn convert(&self, bytes: u64) -> f64 {
        match self {
            DataSize::Byte => bytes as f64,
            DataSize::Kilobyte => bytes as f64 / 1024.0,
            DataSize::Megabyte => bytes as f64 / (1024.0 * 1024.0),
            DataSize::Gigabyte => bytes as f64 / (1024.0 * 1024.0 * 1024.0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSample {
    #[serde(with = "timestamp_serde")]
    pub timestamp: SystemTime,
    pub bytes_received: u64,
    pub bytes_sent: u64,
    pub bytes_per_second_in: f64,
    pub bytes_per_second_out: f64,
    // Add human readable converted values
    pub transfer_rate_in: String,
    pub transfer_rate_out: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlySample {
    #[serde(with = "timestamp_serde")]
    pub timestamp: SystemTime,
    pub avg_bytes_per_second_in: f64,
    pub avg_bytes_per_second_out: f64,
    pub total_bytes_received: u64,
    pub total_bytes_sent: u64,
}

#[derive(Debug)]
pub struct NetworkAnalytics {
    pub interface_name: String,
    pub current_sample: NetworkSample,
    recent_samples: VecDeque<NetworkSample>,
    hourly_samples: VecDeque<HourlySample>,
    max_samples: usize,
    max_hourly_samples: usize,
    last_bytes_received: u64,
    last_bytes_sent: u64,
    last_update: SystemTime,
    data_size: DataSize,
}

impl NetworkAnalytics {
    pub fn new(interface_name: String, data_size: DataSize) -> Self {
        let now = SystemTime::now();
        let initial_sample = NetworkSample {
            timestamp: now,
            bytes_received: 0,
            bytes_sent: 0,
            bytes_per_second_in: 0.0,
            bytes_per_second_out: 0.0,
            transfer_rate_in: "0 KB/s".to_string(),
            transfer_rate_out: "0 KB/s".to_string(),
        };

        Self {
            interface_name,
            current_sample: initial_sample,
            recent_samples: VecDeque::with_capacity(60),
            hourly_samples: VecDeque::with_capacity(24),
            max_samples: 60,
            max_hourly_samples: 24,
            last_bytes_received: 0,
            last_bytes_sent: 0,
            last_update: now,
            data_size,
        }
    }

    pub fn update_from_sysinfo(&mut self, bytes_received: u64, bytes_sent: u64) {
        let now = SystemTime::now();

        let duration = now
            .duration_since(self.last_update)
            .unwrap_or(Duration::from_secs(1));
        let duration_secs = duration.as_secs_f64();

        // Calculate rates
        let bytes_per_second_in =
            (bytes_received - self.last_bytes_received) as f64 / duration_secs;
        let bytes_per_second_out = (bytes_sent - self.last_bytes_sent) as f64 / duration_secs;

        // Convert to specified data size and format string
        let rate_in = self.data_size.convert(bytes_per_second_in as u64);
        let rate_out = self.data_size.convert(bytes_per_second_out as u64);

        let transfer_rate_in = format!("{:.2} {}/s", rate_in, self.get_size_suffix());
        let transfer_rate_out = format!("{:.2} {}/s", rate_out, self.get_size_suffix());

        let new_sample = NetworkSample {
            timestamp: now,
            bytes_received,
            bytes_sent,
            bytes_per_second_in,
            bytes_per_second_out,
            transfer_rate_in,
            transfer_rate_out,
        };

        // Update recent samples
        self.recent_samples.push_back(new_sample.clone());
        if self.recent_samples.len() > self.max_samples {
            self.recent_samples.pop_front();
        }

        // Update current state
        self.current_sample = new_sample;
        self.last_bytes_received = bytes_received;
        self.last_bytes_sent = bytes_sent;
        self.last_update = now;
    }

    fn aggregate_hourly_data(&mut self, timestamp: SystemTime) {
        if self.recent_samples.is_empty() {
            return;
        }

        // Calculate averages from recent samples
        let samples_count = self.recent_samples.len() as f64;
        let avg_bytes_per_second_in: f64 = self
            .recent_samples
            .iter()
            .map(|s| s.bytes_per_second_in)
            .sum::<f64>()
            / samples_count;

        let avg_bytes_per_second_out: f64 = self
            .recent_samples
            .iter()
            .map(|s| s.bytes_per_second_out)
            .sum::<f64>()
            / samples_count;

        // Create hourly sample
        let hourly_sample = HourlySample {
            timestamp,
            avg_bytes_per_second_in,
            avg_bytes_per_second_out,
            total_bytes_received: self.current_sample.bytes_received,
            total_bytes_sent: self.current_sample.bytes_sent,
        };

        // Store hourly sample
        self.hourly_samples.push_back(hourly_sample);
        if self.hourly_samples.len() > self.max_hourly_samples {
            self.hourly_samples.pop_front();
        }
    }

    fn get_size_suffix(&self) -> &'static str {
        match self.data_size {
            DataSize::Byte => "B",
            DataSize::Kilobyte => "KB",
            DataSize::Megabyte => "MB",
            DataSize::Gigabyte => "GB",
        }
    }

    pub fn get_recent_samples(&self) -> Vec<&NetworkSample> {
        self.recent_samples.iter().collect()
    }

    pub fn get_hourly_samples(&self) -> Vec<&HourlySample> {
        self.hourly_samples.iter().collect()
    }

    pub fn get_metrics(&self) -> NetworkMetrics {
        NetworkMetrics {
            interface: self.interface_name.clone(),
            current: self.current_sample.clone(),
            recent: self.recent_samples.iter().cloned().collect(),
        }
    }
}

// Return structure for the endpoints
#[derive(Debug, Serialize)]
pub struct NetworkMetrics {
    pub interface: String,
    pub current: NetworkSample,
    pub recent: Vec<NetworkSample>,
}

// Timestamp serialization module (as before)
mod timestamp_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let unix_timestamp = time
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        serializer.serialize_f64(unix_timestamp)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let unix_timestamp = f64::deserialize(deserializer)?;
        let duration = Duration::from_secs_f64(unix_timestamp);
        Ok(UNIX_EPOCH + duration)
    }
}
