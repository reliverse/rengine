import { AlertCircle, RefreshCw } from "lucide-react";
import { Component, type ReactNode, useState } from "react";
import { Alert, AlertDescription, AlertTitle } from "~/components/ui/alert";
import { Button } from "~/components/ui/button";

interface ErrorBoundaryProps {
  children: ReactNode;
  fallback?: ReactNode;
  onError?: (error: Error, errorInfo: React.ErrorInfo) => void;
}

function ErrorBoundary({ children, fallback, onError }: ErrorBoundaryProps) {
  const [hasError, setHasError] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  if (hasError && error) {
    if (fallback) {
      return fallback;
    }

    return (
      <div className="flex min-h-[400px] flex-col items-center justify-center p-6">
        <div className="w-full max-w-lg space-y-6">
          <Alert className="border-destructive/30 bg-destructive/10">
            <AlertCircle className="h-4 w-4" />
            <AlertTitle>Something went wrong</AlertTitle>
            <AlertDescription className="mt-2">
              An unexpected error occurred. Please try refreshing the page.
            </AlertDescription>
          </Alert>

          <div className="flex flex-col gap-2">
            <Button
              className="w-full"
              onClick={() => window.location.reload()}
              variant="default"
            >
              <RefreshCw className="mr-2 h-4 w-4" />
              Reload Page
            </Button>
          </div>

          {import.meta.env.DEV && (
            <details className="rounded-lg border bg-muted/50 p-3">
              <summary className="cursor-pointer font-medium text-sm">
                Technical Details
              </summary>
              <pre className="mt-2 whitespace-pre-wrap text-muted-foreground text-xs">
                {error.message}
                {error.stack && `\n\n${error.stack}`}
              </pre>
            </details>
          )}
        </div>
      </div>
    );
  }

  return (
    <ErrorBoundaryWrapper
      onError={(error, errorInfo) => {
        setHasError(true);
        setError(error);
        onError?.(error, errorInfo);
      }}
    >
      {children}
    </ErrorBoundaryWrapper>
  );
}
interface ErrorBoundaryWrapperProps {
  children: ReactNode;
  onError: (error: Error, errorInfo: React.ErrorInfo) => void;
}

class ErrorBoundaryWrapper extends Component<ErrorBoundaryWrapperProps> {
  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    this.props.onError(error, errorInfo);
  }

  render() {
    return this.props.children;
  }
}

export { ErrorBoundary };
