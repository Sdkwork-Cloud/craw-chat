import { useRef, useCallback, useEffect } from "react";

export interface LongPressHandlers {
  onPointerDown: () => void;
  onPointerUp: () => void;
  onPointerLeave: () => void;
}

export interface UseLongPressOptions {
  delay?: number;
  onLongPress: () => void;
  onLongPressEnd?: () => void;
}

export function useLongPress(options: UseLongPressOptions): LongPressHandlers {
  const { delay = 500, onLongPress, onLongPressEnd } = options;
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const start = useCallback(() => {
    if (timeoutRef.current) clearTimeout(timeoutRef.current);
    timeoutRef.current = setTimeout(() => {
      onLongPress();
    }, delay);
  }, [delay, onLongPress]);

  const clear = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
      onLongPressEnd?.();
    }
  }, [onLongPressEnd]);

  useEffect(() => {
    return () => {
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
    };
  }, []);

  return {
    onPointerDown: start,
    onPointerUp: clear,
    onPointerLeave: clear,
  };
}
