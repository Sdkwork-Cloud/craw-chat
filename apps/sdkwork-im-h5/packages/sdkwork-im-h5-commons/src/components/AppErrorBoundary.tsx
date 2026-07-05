import React, { Component, type ErrorInfo, type ReactNode } from 'react';

export interface AppErrorBoundaryProps {
  children: ReactNode;
  /**
   * Optional custom fallback node rendered when an error is caught.
   * If omitted, the default localized fallback is used.
   */
  fallback?: ReactNode;
  /**
   * Optional callback invoked when an error is caught. Use this to wire
   * telemetry / monitoring sinks.
   */
  onError?: (error: Error, info: ErrorInfo) => void;
  /**
   * When any value in this array changes, the boundary automatically resets
   * its error state. Pass `[location.pathname]` to reset on route
   * navigation so a transient render error on one page does not persist
   * after the user navigates away.
   */
  resetKeys?: ReadonlyArray<unknown>;
}

interface AppErrorBoundaryState {
  error: Error | null;
}

/**
 * P2-24: React error boundary for the IM H5 application.
 *
 * The H5 app previously had zero error boundary coverage, meaning any
 * render-phase exception would bubble to React's root and white-screen
 * the entire mobile view. This class component provides:
 * - Custom `fallback` rendering.
 * - `onError` telemetry hook.
 * - `resetKeys` for automatic recovery on navigation.
 */
export class AppErrorBoundary extends Component<AppErrorBoundaryProps, AppErrorBoundaryState> {
  state: AppErrorBoundaryState = { error: null };

  static getDerivedStateFromError(error: Error): AppErrorBoundaryState {
    return { error };
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    this.props.onError?.(error, info);
  }

  componentDidUpdate(prevProps: AppErrorBoundaryProps) {
    if (this.state.error && this.props.resetKeys) {
      const changed = this.props.resetKeys.some(
        (key, index) => !Object.is(key, prevProps.resetKeys?.[index]),
      );
      if (changed) {
        this.setState({ error: null });
      }
    }
  }

  render() {
    if (this.state.error) {
      if (this.props.fallback) {
        return this.props.fallback;
      }
      return <DefaultErrorFallback error={this.state.error} />;
    }
    return this.props.children;
  }
}

/**
 * Mobile-optimized default fallback. Uses inline styles to avoid
 * depending on a CSS framework being loaded — critical because the
 * error may occur before the shell CSS bundle finishes loading.
 */
export const DefaultErrorFallback: React.FC<{ error: Error }> = ({ error }) => {
  const isDev = import.meta.env?.DEV === true;
  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        minHeight: '100vh',
        padding: '24px',
        background: '#0a0a0a',
        color: '#d4d4d8',
        textAlign: 'center',
        fontFamily: 'system-ui, -apple-system, sans-serif',
      }}
    >
      <div style={{ maxWidth: '320px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '16px', fontWeight: 600, color: '#fafafa' }}>
          页面发生错误 / Something went wrong
        </div>
        <div style={{ fontSize: '13px', color: '#a1a1aa', wordBreak: 'break-word' }}>
          {error.message}
        </div>
        {isDev && error.stack ? (
          <pre
            style={{
              maxHeight: '120px',
              overflow: 'auto',
              background: 'rgba(0,0,0,0.4)',
              padding: '8px',
              borderRadius: '6px',
              fontSize: '11px',
              color: '#71717a',
              textAlign: 'left',
              whiteSpace: 'pre-wrap',
              wordBreak: 'break-word',
            }}
          >
            {error.stack}
          </pre>
        ) : null}
        <div style={{ display: 'flex', gap: '12px', justifyContent: 'center', paddingTop: '8px' }}>
          <button
            type="button"
            onClick={() => window.location.reload()}
            style={{
              background: '#4f46e5',
              color: '#fff',
              border: 'none',
              borderRadius: '6px',
              padding: '8px 16px',
              fontSize: '12px',
              fontWeight: 500,
              cursor: 'pointer',
            }}
          >
            重试 / Retry
          </button>
          <a
            href="#/chat/inbox"
            style={{
              border: '1px solid rgba(255,255,255,0.1)',
              color: '#d4d4d8',
              borderRadius: '6px',
              padding: '8px 16px',
              fontSize: '12px',
              fontWeight: 500,
              textDecoration: 'none',
            }}
          >
            返回首页 / Home
          </a>
        </div>
      </div>
    </div>
  );
};
