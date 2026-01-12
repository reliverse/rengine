export type LogLevel = 0 | 1 | 2 | 3 | 4 | 5;

export const LogLevel = {
  Trace: 0 as LogLevel,
  Debug: 1 as LogLevel,
  Info: 2 as LogLevel,
  Warning: 3 as LogLevel,
  Error: 4 as LogLevel,
  Critical: 5 as LogLevel,
} as const;

export type LogCategory =
  | "System"
  | "Ui"
  | "FileIo"
  | "Tool"
  | "Memory"
  | "Performance"
  | "UserAction"
  | "Error";

export const LogCategory = {
  System: "System" as LogCategory,
  Ui: "Ui" as LogCategory,
  FileIo: "FileIo" as LogCategory,
  Tool: "Tool" as LogCategory,
  Memory: "Memory" as LogCategory,
  Performance: "Performance" as LogCategory,
  UserAction: "UserAction" as LogCategory,
  Error: "Error" as LogCategory,
} as const;

export interface LogEntry {
  timestamp: string;
  level: LogLevel;
  category: LogCategory;
  message: string;
  source?: string;
  data?: any;
  thread_name: string;
  session_id: string;
}

export interface PerformanceTimer {
  operation: string;
  start_time: string;
  duration_ms?: number;
  details?: any;
}

export interface MemoryStats {
  used_mb: number;
  total_mb: number;
  available_mb: number;
  process_memory_mb: number;
  timestamp: string;
}

export interface DebugSessionInfo {
  session_id: string;
  session_start: string;
  log_file_path: string;
  active_timers_count: number;
}

export interface DebugLogger {
  log(
    level: LogLevel,
    category: LogCategory,
    message: string,
    data?: any
  ): Promise<void>;
  trace(category: LogCategory, message: string, data?: any): Promise<void>;
  debug(category: LogCategory, message: string, data?: any): Promise<void>;
  info(category: LogCategory, message: string, data?: any): Promise<void>;
  warning(category: LogCategory, message: string, data?: any): Promise<void>;
  error(category: LogCategory, message: string, data?: any): Promise<void>;
  critical(category: LogCategory, message: string, data?: any): Promise<void>;

  startPerformanceTimer(operation: string): Promise<string>;
  endPerformanceTimer(timerId: string, details?: any): Promise<void>;

  getMemoryStats(): Promise<MemoryStats>;
  getSessionInfo(): Promise<DebugSessionInfo>;

  logUserAction(action: string, details?: any): Promise<void>;
  logToolOperation(
    toolName: string,
    operation: string,
    details?: any
  ): Promise<void>;
  logFileOperation(
    operation: string,
    filePath: string,
    success: boolean,
    details?: any
  ): Promise<void>;
  logMemoryUsage(component: string, memoryMb: number): Promise<void>;
}

export interface PerformanceTimerResult {
  timerId: string;
  endTimer: (details?: any) => Promise<void>;
}
