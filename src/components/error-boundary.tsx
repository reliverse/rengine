import React from "react";

interface ErrorBoundaryState {
  hasError: boolean;
  error?: Error;
}

interface ErrorBoundaryProps {
  children: React.ReactNode;
  fallback?: React.ComponentType<{ error?: Error; resetError: () => void }>;
}

class ErrorBoundary extends React.Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    console.error("ErrorBoundary caught an error:", error, errorInfo);
  }

  resetError = () => {
    this.setState({ hasError: false, error: undefined });
  };

  render() {
    if (this.state.hasError) {
      const FallbackComponent = this.props.fallback || DefaultErrorFallback;
      return <FallbackComponent error={this.state.error} resetError={this.resetError} />;
    }

    return this.props.children;
  }
}

function DefaultErrorFallback({ error, resetError }: { error?: Error; resetError: () => void }) {
  return (
    <div className="w-full h-full bg-gray-900 flex items-center justify-center">
      <div className="text-center text-white">
        <h2 className="text-xl font-semibold mb-4">Something went wrong</h2>
        <p className="text-gray-400 mb-4">
          The 3D editor encountered an error and couldn't render properly.
        </p>
        {error && (
          <details className="mb-4 text-left">
            <summary className="cursor-pointer text-sm text-gray-500">Error details</summary>
            <pre className="mt-2 text-xs text-red-400 bg-gray-800 p-2 rounded overflow-auto">
              {error.message}
            </pre>
          </details>
        )}
        <button
          className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded text-white"
          onClick={resetError}
        >
          Try Again
        </button>
      </div>
    </div>
  );
}

export default ErrorBoundary;
