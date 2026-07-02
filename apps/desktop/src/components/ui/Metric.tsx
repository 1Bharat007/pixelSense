import * as React from "react"
import { cn } from "../../lib/utils"

export interface MetricProps extends React.HTMLAttributes<HTMLDivElement> {
  label: string;
  value: React.ReactNode;
  icon?: React.ReactNode;
  subValue?: string;
}

export const Metric = React.forwardRef<HTMLDivElement, MetricProps>(
  ({ label, value, icon, subValue, className, ...props }, ref) => {
    return (
      <div ref={ref} className={cn("flex flex-col gap-1", className)} {...props}>
        <div className="flex items-center justify-between text-muted-foreground mb-1">
          <span className="text-sm font-medium uppercase tracking-wider">{label}</span>
          {icon && <span className="text-muted-foreground/80">{icon}</span>}
        </div>
        <div className="flex items-baseline gap-2">
          <span className="text-2xl font-semibold tracking-tight text-foreground">{value}</span>
          {subValue && <span className="text-sm text-muted-foreground">{subValue}</span>}
        </div>
      </div>
    )
  }
)
Metric.displayName = "Metric"
