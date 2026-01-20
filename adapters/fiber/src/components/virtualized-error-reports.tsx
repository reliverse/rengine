// Virtualized Error Reports Component
import { Bug, Clock, ExternalLink } from "lucide-react";
import { memo } from "react";
import { VList } from "virtua";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import type { ErrorReport } from "~/lib/error-reporting";
import { cn } from "~/lib/utils";

interface VirtualizedErrorReportsProps {
  reports: ErrorReport[];
  height: number;
  className?: string;
}

export const VirtualizedErrorReports = memo(function VirtualizedErrorReports({
  reports,
  height,
  className,
}: VirtualizedErrorReportsProps) {
  const getSeverityColor = (severity: ErrorReport["severity"]) => {
    switch (severity) {
      case "critical":
        return "text-red-600 bg-red-50 border-red-200";
      case "high":
        return "text-orange-600 bg-orange-50 border-orange-200";
      case "medium":
        return "text-yellow-600 bg-yellow-50 border-yellow-200";
      case "low":
        return "text-blue-600 bg-blue-50 border-blue-200";
      default:
        return "text-gray-600 bg-gray-50 border-gray-200";
    }
  };

  const getCategoryIcon = (category: ErrorReport["category"]) => {
    switch (category) {
      case "network":
        return "🌐";
      case "auth":
        return "🔐";
      case "api":
        return "🔌";
      case "ui":
        return "🖥️";
      case "validation":
        return "✅";
      default:
        return "❓";
    }
  };

  if (reports.length === 0) {
    return (
      <div
        className={cn(
          "flex items-center justify-center text-muted-foreground",
          className
        )}
        style={{ height }}
      >
        <div className="text-center">
          <div className="mb-2 text-2xl">✅</div>
          <div className="text-sm">No errors reported</div>
        </div>
      </div>
    );
  }

  return (
    <div className={cn("overflow-hidden rounded-lg border", className)}>
      {/* Header */}
      <div className="flex items-center gap-2 border-b bg-muted/50 px-3 py-2">
        <Bug className="h-4 w-4" />
        <div className="font-medium text-sm">Error Reports</div>
        <Badge className="text-xs" variant="secondary">
          {reports.length}
        </Badge>
      </div>

      {/* Virtualized List */}
      <VList
        className="scrollbar-thin scrollbar-thumb-muted scrollbar-track-transparent"
        style={{ height: height - 40 }}
      >
        {reports.map((report) => (
          <div
            className="group border-border border-b p-3 last:border-b-0 hover:bg-muted/20"
            key={report.id}
          >
            <div className="flex items-start gap-3">
              {/* Category Icon */}
              <div className="flex h-8 w-8 items-center justify-center rounded-full bg-muted text-sm">
                {getCategoryIcon(report.category)}
              </div>

              <div className="min-w-0 flex-1">
                {/* Error Message */}
                <div
                  className="truncate font-medium text-sm"
                  title={report.message}
                >
                  {report.message}
                </div>

                {/* Metadata */}
                <div className="mt-1 flex items-center gap-2 text-muted-foreground text-xs">
                  <div className="flex items-center gap-1">
                    <Clock className="h-3 w-3" />
                    {new Date(report.timestamp).toLocaleString()}
                  </div>
                  <Badge
                    className={cn(
                      "px-1 py-0 text-xs",
                      getSeverityColor(report.severity)
                    )}
                    variant="outline"
                  >
                    {report.severity}
                  </Badge>
                  <span className="capitalize">{report.category}</span>
                </div>

                {/* Stack Trace Preview */}
                {report.stack && (
                  <details className="mt-2">
                    <summary className="cursor-pointer text-muted-foreground text-xs hover:text-foreground">
                      Stack Trace
                    </summary>
                    <pre className="mt-1 overflow-x-auto whitespace-pre-wrap rounded bg-muted p-2 text-xs">
                      {report.stack.slice(0, 500)}
                      {report.stack.length > 500 && "..."}
                    </pre>
                  </details>
                )}
              </div>

              {/* Actions */}
              <Button
                className="h-6 w-6 p-0 opacity-0 group-hover:opacity-100"
                onClick={() => {
                  // Could open detailed error view
                  console.log("View error details:", report.id);
                }}
                size="sm"
                variant="ghost"
              >
                <ExternalLink className="h-3 w-3" />
              </Button>
            </div>
          </div>
        ))}
      </VList>
    </div>
  );
});
