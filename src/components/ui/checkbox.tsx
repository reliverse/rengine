import * as React from "react";
import { cn } from "~/lib/utils";

export interface CheckboxProps extends React.InputHTMLAttributes<HTMLInputElement> {}

const Checkbox = ({
  className,
  ref,
  ...props
}: CheckboxProps & { ref?: React.RefObject<HTMLInputElement | null> }) => (
  <input
    className={cn(
      "h-4 w-4 rounded border border-input ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
      className,
    )}
    ref={ref}
    type="checkbox"
    {...props}
  />
);
Checkbox.displayName = "Checkbox";

export { Checkbox };
