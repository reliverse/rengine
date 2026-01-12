import { invoke } from "@tauri-apps/api/core";
import {
  type DebugLogger,
  type DebugSessionInfo,
  LogCategory,
  LogLevel,
  type MemoryStats,
  type PerformanceTimerResult,
} from "~/types/debug";

class RengineDebugLogger implements DebugLogger {
  async log(
    level: LogLevel,
    category: LogCategory,
    message: string,
    data?: any
  ): Promise<void> {
    await invoke("log_debug_message", {
      level,
      category: category as string,
      message,
      data: data ? JSON.parse(JSON.stringify(data)) : null,
    });
  }

  async trace(
    category: LogCategory,
    message: string,
    data?: any
  ): Promise<void> {
    await this.log(LogLevel.Trace, category, message, data);
  }

  async debug(
    category: LogCategory,
    message: string,
    data?: any
  ): Promise<void> {
    await this.log(LogLevel.Debug, category, message, data);
  }

  async info(
    category: LogCategory,
    message: string,
    data?: any
  ): Promise<void> {
    await this.log(LogLevel.Info, category, message, data);
  }

  async warning(
    category: LogCategory,
    message: string,
    data?: any
  ): Promise<void> {
    await this.log(LogLevel.Warning, category, message, data);
  }

  async error(
    category: LogCategory,
    message: string,
    data?: any
  ): Promise<void> {
    await this.log(LogLevel.Error, category, message, data);
  }

  async critical(
    category: LogCategory,
    message: string,
    data?: any
  ): Promise<void> {
    await this.log(LogLevel.Critical, category, message, data);
  }

  async startPerformanceTimer(operation: string): Promise<string> {
    return await invoke("start_performance_timer", { operation });
  }

  async endPerformanceTimer(timerId: string, details?: any): Promise<void> {
    await invoke("end_performance_timer", {
      timerId,
      details: details ? JSON.parse(JSON.stringify(details)) : null,
    });
  }

  async getMemoryStats(): Promise<MemoryStats> {
    return await invoke("get_memory_stats");
  }

  async getSessionInfo(): Promise<DebugSessionInfo> {
    return await invoke("get_debug_session_info");
  }

  async logUserAction(action: string, details?: any): Promise<void> {
    await invoke("log_user_action", {
      action,
      details: details ? JSON.parse(JSON.stringify(details)) : null,
    });
  }

  async logToolOperation(
    toolName: string,
    operation: string,
    details?: any
  ): Promise<void> {
    await invoke("log_tool_operation", {
      toolName,
      operation,
      details: details ? JSON.parse(JSON.stringify(details)) : null,
    });
  }

  async logFileOperation(
    operation: string,
    filePath: string,
    success: boolean,
    details?: any
  ): Promise<void> {
    const message = `File ${operation}: ${filePath}`;
    const level = success ? LogLevel.Info : LogLevel.Error;
    await this.log(level, LogCategory.FileIo, message, details);
  }

  async logMemoryUsage(component: string, memoryMb: number): Promise<void> {
    await this.debug(
      LogCategory.Memory,
      `Memory usage - ${component}: ${memoryMb.toFixed(2)} MB`,
      {
        component,
        memory_mb: memoryMb,
      }
    );
  }

  // Convenience methods for performance timing
  async timeOperation<T>(
    operation: string,
    fn: () => Promise<T>,
    details?: any
  ): Promise<T> {
    const timerId = await this.startPerformanceTimer(operation);
    try {
      const result = await fn();
      await this.endPerformanceTimer(timerId, { ...details, success: true });
      return result;
    } catch (error) {
      await this.endPerformanceTimer(timerId, {
        ...details,
        success: false,
        error: error instanceof Error ? error.message : String(error),
      });
      throw error;
    }
  }

  async createPerformanceTimer(
    operation: string
  ): Promise<PerformanceTimerResult> {
    const timerId = await this.startPerformanceTimer(operation);
    return {
      timerId,
      endTimer: async (details?: any) => {
        await this.endPerformanceTimer(timerId, details);
      },
    };
  }

  // Logging decorators/helpers
  async logFunctionCall(
    funcName: string,
    args: any[] = [],
    kwargs: Record<string, any> = {}
  ): Promise<void> {
    await this.trace(LogCategory.System, `Function call: ${funcName}`, {
      function: funcName,
      args: JSON.stringify(args).slice(0, 200),
      kwargs: JSON.stringify(kwargs).slice(0, 200),
    });
  }

  async logException(
    category: LogCategory,
    message: string,
    error: Error
  ): Promise<void> {
    await this.error(category, message, {
      exception_type: error.constructor.name,
      exception_message: error.message,
      stack_trace: error.stack,
    });
  }
}

// Global debug logger instance
let globalDebugLogger: RengineDebugLogger | null = null;

export function getDebugLogger(): DebugLogger {
  if (!globalDebugLogger) {
    globalDebugLogger = new RengineDebugLogger();
  }
  return globalDebugLogger;
}

// Convenience functions for quick logging
export const debugLog = {
  trace: (category: LogCategory, message: string, data?: any) =>
    getDebugLogger().trace(category, message, data),

  debug: (category: LogCategory, message: string, data?: any) =>
    getDebugLogger().debug(category, message, data),

  info: (category: LogCategory, message: string, data?: any) =>
    getDebugLogger().info(category, message, data),

  warning: (category: LogCategory, message: string, data?: any) =>
    getDebugLogger().warning(category, message, data),

  error: (category: LogCategory, message: string, data?: any) =>
    getDebugLogger().error(category, message, data),

  critical: (category: LogCategory, message: string, data?: any) =>
    getDebugLogger().critical(category, message, data),

  userAction: (action: string, details?: any) =>
    getDebugLogger().logUserAction(action, details),

  toolOperation: (toolName: string, operation: string, details?: any) =>
    getDebugLogger().logToolOperation(toolName, operation, details),

  fileOperation: (
    operation: string,
    filePath: string,
    success: boolean,
    details?: any
  ) => getDebugLogger().logFileOperation(operation, filePath, success, details),

  memoryUsage: (component: string, memoryMb: number) =>
    getDebugLogger().logMemoryUsage(component, memoryMb),

  // Note: timeOperation and createTimer not implemented in Rust backend yet
  // timeOperation: async <T>(
  //   operation: string,
  //   fn: () => Promise<T>,
  //   details?: any
  // ): Promise<T> => getDebugLogger().timeOperation(operation, fn, details),

  // createTimer: (operation: string) =>
  //   getDebugLogger().createPerformanceTimer(operation),

  getMemoryStats: () => getDebugLogger().getMemoryStats(),

  getSessionInfo: () => getDebugLogger().getSessionInfo(),
};

// Helper function to create a logged version of a function
export function withLogging<T extends (...args: any[]) => any>(
  fn: T,
  funcName?: string
): T {
  const name = funcName || fn.name || "anonymous";
  const logger = getDebugLogger();

  return (async (...args: any[]) => {
    // logger.logFunctionCall(name, args.slice(0, 5), {}); // Limit args for performance - not implemented

    const timerId = await logger.startPerformanceTimer(name);

    try {
      const result = await fn(...args);
      logger.endPerformanceTimer(timerId, { success: true });
      return result;
    } catch (error) {
      logger.endPerformanceTimer(timerId, {
        success: false,
        error: error instanceof Error ? error.message : String(error),
      });
      throw error;
    }
  }) as T;
}

// Helper for logging exceptions with proper error handling
// Note: logException not implemented in Rust backend yet
// export async function logException(
//   category: LogCategory,
//   message: string,
//   error: unknown
// ): Promise<void> {
//   if (error instanceof Error) {
//     await getDebugLogger().logException(category, message, error);
//   } else {
//     await debugLog.error(category, message, {
//       error_type: typeof error,
//       error_value: String(error),
//     });
//   }
// }
