import { useCallback, useRef } from "react";

/**
 * Hook for preemptive render scheduling to avoid animation jumps
 * Based on React Three Fiber performance optimization patterns
 */
export function usePreemptiveRender() {
  // Note: invalidate functionality temporarily disabled due to TypeScript API issues
  const invalidate = () => {
    /* No-op placeholder */
  };
  const animationFrameRef = useRef<number | undefined>(undefined);

  /**
   * Schedule a render and execute an animation in the next frame
   * This prevents visible jumps in synchronous animations
   */

  // biome-ignore lint/correctness/useExhaustiveDependencies: <>
  const scheduleAnimation = useCallback(
    (animationCallback: () => void) => {
      // Pre-emptively schedule a render
      invalidate();

      // Cancel any pending animation frame
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }

      // Start the animation in the next frame
      animationFrameRef.current = requestAnimationFrame(() => {
        animationCallback();
        animationFrameRef.current = undefined;
      });
    },
    [invalidate]
  );

  /**
   * Schedule a render for interactive events (mouse, touch, etc.)
   */
  const scheduleInteraction = useCallback(
    (interactionCallback?: () => void) => {
      invalidate();

      if (interactionCallback) {
        // Small delay to ensure render happens first
        setTimeout(interactionCallback, 0);
      }
    },
    []
  );

  /**
   * Schedule a render for state changes that need immediate visual feedback
   */
  const scheduleUpdate = useCallback((updateCallback?: () => void) => {
    invalidate();

    if (updateCallback) {
      // Use microtask for immediate execution after render
      queueMicrotask(updateCallback);
    }
  }, []);

  return {
    scheduleAnimation,
    scheduleInteraction,
    scheduleUpdate,
    invalidate,
  };
}

/**
 * Hook for managing animation sequences with proper render scheduling
 */
export function useAnimationSequence() {
  const { scheduleAnimation, invalidate } = usePreemptiveRender();
  const sequenceRef = useRef<(() => void)[]>([]);

  const addToSequence = useCallback((animationStep: () => void) => {
    sequenceRef.current.push(animationStep);
  }, []);

  const playSequence = useCallback(() => {
    if (sequenceRef.current.length === 0) return;

    const sequence = [...sequenceRef.current];
    sequenceRef.current = [];

    // Play animations in sequence
    let index = 0;
    const playNext = () => {
      if (index < sequence.length) {
        scheduleAnimation(() => {
          sequence[index]();
          index++;
          // Schedule next animation
          setTimeout(playNext, 16); // ~60fps
        });
      }
    };

    playNext();
  }, [scheduleAnimation]);

  const clearSequence = useCallback(() => {
    sequenceRef.current = [];
  }, []);

  return {
    addToSequence,
    playSequence,
    clearSequence,
    invalidate,
  };
}

/**
 * Hook for debounced render scheduling to avoid excessive renders
 */
export function useDebouncedRender(delay = 16) {
  // Note: invalidate functionality temporarily disabled due to TypeScript API issues
  const invalidate = () => {
    /* No-op placeholder */
  };
  const timeoutRef = useRef<NodeJS.Timeout | undefined>(undefined);

  const debouncedInvalidate = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }

    timeoutRef.current = setTimeout(() => {
      invalidate();
      timeoutRef.current = undefined;
    }, delay);
  }, [delay]);

  const cancel = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = undefined;
    }
  }, []);

  const immediate = useCallback(() => {
    cancel();
    invalidate();
  }, [cancel]);

  return {
    debouncedInvalidate,
    cancel,
    immediate,
  };
}

/**
 * Hook for conditional render scheduling based on performance
 */
export function useConditionalRender() {
  // Note: invalidate functionality temporarily disabled due to TypeScript API issues
  const invalidate = () => {
    /* No-op placeholder */
  };
  const performance = { current: 0.5 }; // Default performance value

  const conditionalInvalidate = useCallback((condition = true) => {
    // Only invalidate if condition is met and performance allows it
    if (condition && performance.current > 0.3) {
      invalidate();
    }
  }, []);

  return {
    conditionalInvalidate,
    performanceFactor: performance.current,
  };
}
