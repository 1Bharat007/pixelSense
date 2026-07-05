import * as React from 'react';
import { motion, type HTMLMotionProps } from "framer-motion"
import { cn } from "../../lib/utils"

export interface CardProps extends HTMLMotionProps<"div"> {
  interactive?: boolean;
}

export const Card = React.forwardRef<HTMLDivElement, CardProps>(
  ({ className, interactive, children, ...props }, ref) => {
    return (
      <motion.div
        ref={ref}
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.4, ease: "easeOut" }}
        whileHover={interactive ? { y: -2, scale: 1.005 } : undefined}
        whileTap={interactive ? { scale: 0.98 } : undefined}
        className={cn(
          "rounded-xl border border-border bg-card text-card-foreground shadow-sm p-6 flex flex-col gap-4 relative overflow-hidden",
          interactive && "cursor-pointer hover:border-ring/50 transition-colors focus-visible:outline-ring",
          className
        )}
        {...props}
      >
        {children}
      </motion.div>
    )
  }
)
Card.displayName = "Card"
