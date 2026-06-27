import React, { Component, type ErrorInfo, type ReactNode } from 'react';

export interface AppErrorBoundaryProps {
  children: ReactNode;
  fallback?: ReactNode;
  onError?: (error: Error, info: ErrorInfo) => void;
}

interface AppErrorBoundaryState {
  error: Error | null;
}

export class AppErrorBoundary extends Component<AppErrorBoundaryProps, AppErrorBoundaryState> {
  state: AppErrorBoundaryState = { error: null };

  static getDerivedStateFromError(error: Error): AppErrorBoundaryState {
    return { error };
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    this.props.onError?.(error, info);
  }

  render() {
    if (this.state.error) {
      if (this.props.fallback) {
        return this.props.fallback;
      }
      return (
        <div className="flex h-full w-full items-center justify-center bg-zinc-950 p-6 text-center text-sm text-zinc-300">
          <div className="max-w-md space-y-2">
            <div className="text-base font-medium text-zinc-100">Something went wrong</div>
            <div className="text-zinc-400">{this.state.error.message}</div>
          </div>
        </div>
      );
    }
    return this.props.children;
  }
}
