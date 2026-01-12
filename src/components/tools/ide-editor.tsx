/**
 * IDE Editor Component
 * Schema-driven editor for GTA item definition files
 */

import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";

interface IdeEditorProps {
  className?: string;
}

export function IdeEditor({ className }: IdeEditorProps) {
  return (
    <div className={className}>
      <Card>
        <CardHeader>
          <CardTitle>IDE Editor</CardTitle>
        </CardHeader>
        <CardContent>
          <p>IDE Editor component is under development.</p>
        </CardContent>
      </Card>
    </div>
  );
}
