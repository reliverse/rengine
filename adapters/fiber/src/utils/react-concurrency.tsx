import * as React from "react";
import { startTransition, useDeferredValue, useTransition } from "react";

// React 18 Concurrency utilities for expensive operations
// Helps maintain stable framerate during heavy computations

/**
 * Hook for managing expensive state updates with concurrency
 */
export function useConcurrentState<T>(initialValue: T) {
  const [isPending, startTransition] = useTransition();
  const [state, setState] = React.useState(initialValue);

  const setConcurrentState = (updater: T | ((prev: T) => T)) => {
    startTransition(() => {
      setState(updater);
    });
  };

  return [state, setConcurrentState, isPending] as const;
}

/**
 * Hook for deferring expensive computations
 */
export function useDeferredComputation<T>(
  computation: () => T,
  ...deps: React.DependencyList
) {
  const deferredComputation = useDeferredValue(computation);
  // biome-ignore lint/correctness/useExhaustiveDependencies: Dependencies are intentionally dynamic for this utility function
  const result = React.useMemo(deferredComputation, deps);

  return result;
}

/**
 * Wrapper for expensive operations that should run concurrently
 */
export function runConcurrent<T>(operation: () => T): Promise<T> {
  return new Promise((resolve) => {
    startTransition(() => {
      const result = operation();
      resolve(result);
    });
  });
}

/**
 * Hook for managing concurrent async operations
 */
export function useConcurrentAsync<T>() {
  const [isPending, startTransition] = useTransition();
  const [data, setData] = React.useState<T | null>(null);
  const [error, setError] = React.useState<Error | null>(null);

  const execute = React.useCallback(
    async (asyncOperation: () => Promise<T>) => {
      setError(null);

      try {
        const result = await asyncOperation();
        startTransition(() => {
          setData(result);
        });
        return result;
      } catch (err) {
        const error = err instanceof Error ? err : new Error(String(err));
        startTransition(() => {
          setError(error);
        });
        throw error;
      }
    },
    []
  );

  return { data, error, isPending, execute };
}

/**
 * Component wrapper for concurrent rendering
 */
export function ConcurrentWrapper({
  children,
  fallback,
}: {
  children: React.ReactNode;
  fallback?: React.ReactNode;
}) {
  const deferredChildren = useDeferredValue(children);

  if (deferredChildren !== children && fallback) {
    return <>{fallback}</>;
  }

  return <>{deferredChildren}</>;
}

/**
 * Hook for batching multiple updates concurrently
 */
export function useConcurrentBatch() {
  const [isPending, startTransition] = useTransition();

  const batchUpdate = React.useCallback((updates: Array<() => void>) => {
    startTransition(() => {
      for (const update of updates) {
        update();
      }
    });
  }, []);

  return { batchUpdate, isPending };
}

/**
 * Hook for debounced concurrent updates
 */
export function useDebouncedConcurrentUpdate(delay = 100) {
  const [isPending, startTransition] = useTransition();
  const timeoutRef = React.useRef<NodeJS.Timeout | undefined>(undefined);

  const update = React.useCallback(
    (updater: () => void) => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }

      timeoutRef.current = setTimeout(() => {
        startTransition(updater);
      }, delay);
    },
    [delay]
  );

  React.useEffect(() => {
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, []);

  return { update, isPending };
}

/**
 * Performance-aware concurrent operation
 * Only uses concurrency when performance allows it
 */
export function usePerformanceAwareConcurrency(performanceFactor: number) {
  const shouldUseConcurrency = performanceFactor > 0.6; // Only use concurrency when performance is decent

  const concurrentUpdate = React.useCallback(
    (updater: () => void) => {
      if (shouldUseConcurrency) {
        startTransition(updater);
      } else {
        updater();
      }
    },
    [shouldUseConcurrency]
  );

  return { concurrentUpdate, isUsingConcurrency: shouldUseConcurrency };
}

/**
 * Concurrent reducer for complex state management
 */
export function useConcurrentReducer<S, A>(
  reducer: React.Reducer<S, A>,
  initialState: S
) {
  const [isPending, startTransition] = useTransition();
  const [state, dispatch] = React.useReducer(reducer, initialState);

  const concurrentDispatch = React.useCallback((action: A) => {
    startTransition(() => {
      dispatch(action);
    });
  }, []);

  return [state, concurrentDispatch, isPending] as const;
}
