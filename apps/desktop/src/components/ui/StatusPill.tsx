import * as React from 'react';
import { cn } from "../../lib/utils"

type StatusVariant = "default" | "success" | "warning" | "error" | "disabled";

export interface StatusPillProps extends React.HTMLAttributes<HTMLSpanElement> {
  variant?: StatusVariant;
  pulse?: boolean;
}

export const StatusPill = React.forwardRef<HTMLSpanElement, StatusPillProps>(
  ({ className, variant = "default", pulse = false, children, ...props }, ref) => {
    
    // Using semantic colors instead of raw hex/tailwind colors
    const variants: Record<StatusVariant, string> = {
      default: "bg-secondary text-secondary-foreground border-transparent",
      success: "bg-success/15 text-success-foreground border-success/30",
      warning: "bg-warning/15 text-warning-foreground border-warning/30",
      error: "bg-destructive/15 text-destructive-foreground border-destructive/30",
      disabled: "bg-muted text-muted-foreground border-transparent",
    };

    return (
      <span
        ref={ref}
        className={cn(
          "inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-medium border",
          variants[variant],
          className
        )}
        {...props}
      >
        {pulse && (
          <span className="relative flex h-2 w-2">
            <span className={cn(
              "animate-ping absolute inline-flex h-full w-full rounded-full opacity-75",
              variant === "success" ? "bg-success" :
              variant === "warning" ? "bg-warning" :
              variant === "error" ? "bg-destructive" : "bg-primary"
            )}></span>
            <span className={cn(
              "relative inline-flex rounded-full h-2 w-2",
              variant === "success" ? "bg-success" :
              variant === "warning" ? "bg-warning" :
              variant === "error" ? "bg-destructive" : "bg-primary"
            )}></span>
          </span>
        )}
        {children}
      </span>
    )
  }
)
StatusPill.displayName = "StatusPill"
