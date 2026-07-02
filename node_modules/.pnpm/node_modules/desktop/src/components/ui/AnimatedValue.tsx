import { useEffect, useState } from "react";
import { motion, useSpring, useTransform } from "framer-motion";

interface AnimatedValueProps {
  value: number;
  format?: (val: number) => string;
  className?: string;
}

export function AnimatedValue({ value, format, className }: AnimatedValueProps) {
  const springValue = useSpring(value, {
    stiffness: 100,
    damping: 30,
    restDelta: 0.001
  });
  
  const [display, setDisplay] = useState(format ? format(value) : value.toString());

  useEffect(() => {
    springValue.set(value);
  }, [value, springValue]);

  useEffect(() => {
    return springValue.on("change", (latest) => {
      setDisplay(format ? format(latest) : Math.round(latest).toString());
    });
  }, [springValue, format]);

  return <motion.span className={className}>{display}</motion.span>;
}
