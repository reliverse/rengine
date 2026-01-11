export function LoadingFallback() {
  return (
    <div className="flex h-full items-center justify-center">
      <div className="text-center">
        <div className="mx-auto mb-2 h-8 w-8 animate-spin rounded-full border-primary border-b-2" />
        <p className="text-muted-foreground text-sm">Loading 3D scene...</p>
      </div>
    </div>
  );
}
