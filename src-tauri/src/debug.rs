use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::{Pid, Process, System};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Critical = 5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogCategory {
    System,
    Ui,
    FileIo,
    Tool,
    Memory,
    Performance,
    UserAction,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub category: LogCategory,
    pub message: String,
    pub source: Option<String>,
    pub data: Option<serde_json::Value>,
    pub thread_name: String,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTimer {
    pub operation: String,
    pub start_time: DateTime<Utc>,
    pub duration_ms: Option<f64>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub used_mb: f64,
    pub total_mb: f64,
    pub available_mb: f64,
    pub process_memory_mb: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct DebugLogger {
    session_start: DateTime<Utc>,
    log_file: PathBuf,
    active_timers: Arc<Mutex<HashMap<String, PerformanceTimer>>>,
    system: Arc<Mutex<System>>,
}

impl DebugLogger {
    pub fn new() -> Self {
        let session_start = Utc::now();

        // Create logs directory
        let logs_dir = PathBuf::from("logs");
        if !logs_dir.exists() {
            std::fs::create_dir_all(&logs_dir).ok();
        }

        // Setup log file
        let log_file = logs_dir.join(format!(
            "debug_session_{}.log",
            session_start.format("%Y%m%d_%H%M%S")
        ));

        let logger = Self {
            session_start,
            log_file,
            active_timers: Arc::new(Mutex::new(HashMap::new())),
            system: Arc::new(Mutex::new(System::new_all())),
        };

        // Initialize session
        logger.log(
            LogLevel::Info,
            LogCategory::System,
            "Debug session started".to_string(),
            Some(serde_json::json!({
                "session_id": session_start.to_rfc3339(),
                "rust_version": "unknown", // Would need build script for proper version info
                "platform": std::env::consts::OS,
                "build_date": "unknown",
                "git_sha": "unknown"
            })),
            None,
        );

        logger
    }

    pub fn log(
        &self,
        level: LogLevel,
        category: LogCategory,
        message: String,
        data: Option<serde_json::Value>,
        source: Option<String>,
    ) {
        let thread_name = std::thread::current()
            .name()
            .unwrap_or("unnamed")
            .to_string();

        let entry = LogEntry {
            timestamp: Utc::now(),
            level,
            category,
            message,
            source,
            data,
            thread_name,
            session_id: self.session_start.to_rfc3339(),
        };

        // Write to file
        self.write_to_file(&entry);

        // Print to console with colors
        self.print_to_console(&entry);
    }

    fn write_to_file(&self, entry: &LogEntry) {
        let json_line = match serde_json::to_string(entry) {
            Ok(json) => json,
            Err(_) => return, // Skip if serialization fails
        };

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)
        {
            let _ = writeln!(file, "{}", json_line);
        }
    }

    fn print_to_console(&self, entry: &LogEntry) {
        let timestamp = entry.timestamp.format("%H:%M:%S%.3f");
        let level_str = format!("{:>8}", format!("{:?}", entry.level));
        let category_str = format!("{:>12}", format!("{:?}", entry.category));

        // ANSI color codes
        let (level_color, category_color) = match entry.level {
            LogLevel::Trace => ("\x1b[37m", "\x1b[90m"), // White, Dark gray
            LogLevel::Debug => ("\x1b[36m", "\x1b[96m"), // Cyan, Light cyan
            LogLevel::Info => ("\x1b[32m", "\x1b[92m"),  // Green, Light green
            LogLevel::Warning => ("\x1b[33m", "\x1b[93m"), // Yellow, Light yellow
            LogLevel::Error => ("\x1b[31m", "\x1b[91m"), // Red, Light red
            LogLevel::Critical => ("\x1b[35m", "\x1b[95m"), // Magenta, Light magenta
        };

        print!("[{}] {} {} {}", timestamp, level_color, level_str, category_color);
        print!("{} \x1b[0m{}", category_str, entry.message);

        // Add thread info if not main thread
        if entry.thread_name != "main" {
            print!(" \x1b[90m[{}]\x1b[0m", entry.thread_name);
        }

        // Add data if present
        if let Some(data) = &entry.data {
            if let Ok(pretty_data) = serde_json::to_string_pretty(data) {
                let lines: Vec<&str> = pretty_data.lines().collect();
                if lines.len() > 1 {
                    println!();
                    for line in lines {
                        println!("    \x1b[90m{}\x1b[0m", line);
                    }
                } else {
                    println!(" \x1b[90m{}\x1b[0m", pretty_data);
                }
            }
        } else {
            println!();
        }

        // Add source for errors and critical logs
        if matches!(entry.level, LogLevel::Error | LogLevel::Critical) {
            if let Some(source) = &entry.source {
                println!("    \x1b[90m└─ Source: {}\x1b[0m", source);
            }
        }
    }

    pub fn start_performance_timer(&self, operation: String) -> String {
        let timer_id = format!("{}_{}", operation, Utc::now().timestamp_millis());

        let timer = PerformanceTimer {
            operation: operation.clone(),
            start_time: Utc::now(),
            duration_ms: None,
            details: None,
        };

        {
            let mut timers = self.active_timers.lock().unwrap();
            timers.insert(timer_id.clone(), timer);
        }

        self.log(
            LogLevel::Trace,
            LogCategory::Performance,
            format!("Started timer: {}", operation),
            Some(serde_json::json!({ "timer_id": timer_id })),
            None,
        );

        timer_id
    }

    pub fn end_performance_timer(&self, timer_id: String, details: Option<serde_json::Value>) {
        let mut timers = self.active_timers.lock().unwrap();

        if let Some(mut timer) = timers.remove(&timer_id) {
            let duration = Utc::now().signed_duration_since(timer.start_time);
            let duration_ms = duration.num_milliseconds() as f64;

            timer.duration_ms = Some(duration_ms);
            timer.details = details;

            // Color code based on duration
            let color_code = if duration_ms < 10.0 {
                "\x1b[32m" // Green for fast operations
            } else if duration_ms < 100.0 {
                "\x1b[33m" // Yellow for moderate operations
            } else {
                "\x1b[31m" // Red for slow operations
            };

            println!(
                "\x1b[96m⚡ PERF: {}{} {:.2}ms\x1b[0m",
                color_code, timer.operation, duration_ms
            );

            self.log(
                LogLevel::Info,
                LogCategory::Performance,
                format!("Performance: {} took {:.2}ms", timer.operation, duration_ms),
                Some(serde_json::to_value(&timer).unwrap_or_default()),
                None,
            );
        } else {
            self.log(
                LogLevel::Warning,
                LogCategory::Performance,
                format!("Timer not found: {}", timer_id),
                None,
                None,
            );
        }
    }

    pub fn get_memory_stats(&self) -> MemoryStats {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_all();

        // Get system memory info
        let total_memory = sys.total_memory() as f64 / 1024.0 / 1024.0; // Convert to MB
        let used_memory = sys.used_memory() as f64 / 1024.0 / 1024.0;
        let available_memory = sys.available_memory() as f64 / 1024.0 / 1024.0;

        // Get process memory info
        let process_memory_mb = if let Some(process) = sys.process(Pid::from(std::process::id() as usize)) {
            process.memory() as f64 / 1024.0 / 1024.0
        } else {
            0.0
        };

        MemoryStats {
            used_mb: used_memory,
            total_mb: total_memory,
            available_mb: available_memory,
            process_memory_mb,
            timestamp: Utc::now(),
        }
    }

    pub fn log_memory_usage(&self, component: String, memory_mb: f64) {
        self.log(
            LogLevel::Debug,
            LogCategory::Memory,
            format!("Memory usage - {}: {:.2} MB", component, memory_mb),
            Some(serde_json::json!({
                "component": component,
                "memory_mb": memory_mb
            })),
            None,
        );
    }

    pub fn log_user_action(&self, action: String, details: Option<serde_json::Value>) {
        self.log(
            LogLevel::Info,
            LogCategory::UserAction,
            format!("User action: {}", action),
            details,
            None,
        );
    }

    pub fn log_file_operation(&self, operation: String, file_path: String, success: bool, details: Option<serde_json::Value>) {
        let level = if success { LogLevel::Info } else { LogLevel::Error };
        self.log(
            level,
            LogCategory::FileIo,
            format!("File {}: {}", operation, file_path),
            details,
            None,
        );
    }

    pub fn log_tool_operation(&self, tool_name: String, operation: String, details: Option<serde_json::Value>) {
        self.log(
            LogLevel::Info,
            LogCategory::Tool,
            format!("Tool {}: {}", tool_name, operation),
            details,
            None,
        );
    }

    pub fn get_log_file_path(&self) -> String {
        self.log_file.to_string_lossy().to_string()
    }

    pub fn get_session_info(&self) -> serde_json::Value {
        serde_json::json!({
            "session_id": self.session_start.to_rfc3339(),
            "session_start": self.session_start.to_rfc3339(),
            "log_file_path": self.get_log_file_path(),
            "active_timers_count": self.active_timers.lock().unwrap().len(),
        })
    }
}

// Global debug logger instance
lazy_static::lazy_static! {
    static ref DEBUG_LOGGER: Arc<DebugLogger> = Arc::new(DebugLogger::new());
}

pub fn get_debug_logger() -> Arc<DebugLogger> {
    Arc::clone(&DEBUG_LOGGER)
}

// Convenience macros for logging
#[macro_export]
macro_rules! log_trace {
    ($category:expr, $msg:expr) => {
        $crate::debug::get_debug_logger().log($crate::debug::LogLevel::Trace, $category, $msg.to_string(), None, Some(format!("{}:{}", file!(), line!())));
    };
    ($category:expr, $msg:expr, $data:expr) => {
        $crate::debug::get_debug_logger().log($crate::debug::LogLevel::Trace, $category, $msg.to_string(), Some(serde_json::to_value($data).unwrap_or_default()), Some(format!("{}:{}", file!(), line!())));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($category:expr, $msg:expr) => {
        $crate::debug::get_debug_logger().log($crate::debug::LogLevel::Debug, $category, $msg.to_string(), None, Some(format!("{}:{}", file!(), line!())));
    };
    ($category:expr, $msg:expr, $data:expr) => {
        $crate::debug::get_debug_logger().log($crate::debug::LogLevel::Debug, $category, $msg.to_string(), Some(serde_json::to_value($data).unwrap_or_default()), Some(format!("{}:{}", file!(), line!())));
    };
}

#[macro_export]
macro_rules! log_info {
    ($category:expr, $msg:expr) => {
        $crate::debug::get_debug_logger().log($crate::debug::LogLevel::Info, $category, $msg.to_string(), None, Some(format!("{}:{}", file!(), line!())));
    };
    ($category:expr, $msg:expr, $data:expr) => {
        $crate::debug::get_debug_logger().log($crate::debug::LogLevel::Info, $category, $msg.to_string(), Some(serde_json::to_value($data).unwrap_or_default()), Some(format!("{}:{}", file!(), line!())));
    };
}

#[macro_export]
macro_rules! log_warning {
    ($category:expr, $msg:expr) => {
        $crate::debug::get_debug_logger().log($crate::debug::LogLevel::Warning, $category, $msg.to_string(), None, Some(format!("{}:{}", file!(), line!())));
    };
    ($category:expr, $msg:expr, $data:expr) => {
        $crate::debug::get_debug_logger().log($crate::debug::LogLevel::Warning, $category, $msg.to_string(), Some(serde_json::to_value($data).unwrap_or_default()), Some(format!("{}:{}", file!(), line!())));
    };
}

#[macro_export]
macro_rules! log_error {
    ($category:expr, $msg:expr) => {
        $crate::debug::get_debug_logger().log($crate::debug::LogLevel::Error, $category, $msg.to_string(), None, Some(format!("{}:{}", file!(), line!())));
    };
    ($category:expr, $msg:expr, $data:expr) => {
        $crate::debug::get_debug_logger().log($crate::debug::LogLevel::Error, $category, $msg.to_string(), Some(serde_json::to_value($data).unwrap_or_default()), Some(format!("{}:{}", file!(), line!())));
    };
}

#[macro_export]
macro_rules! log_critical {
    ($category:expr, $msg:expr) => {
        $crate::debug::get_debug_logger().log($crate::debug::LogLevel::Critical, $category, $msg.to_string(), None, Some(format!("{}:{}", file!(), line!())));
    };
    ($category:expr, $msg:expr, $data:expr) => {
        $crate::debug::get_debug_logger().log($crate::debug::LogLevel::Critical, $category, $msg.to_string(), Some(serde_json::to_value($data).unwrap_or_default()), Some(format!("{}:{}", file!(), line!())));
    };
}