import { motion } from "framer-motion";
import { useStore } from "../store/useStore";
import { Sparkles, Sun, Monitor, ShieldCheck, ArrowRight } from "lucide-react";
import { useState } from "react";

export function Onboarding() {
  const { setOnboardingCompleted } = useStore();
  const [step, setStep] = useState(0);

  const completeOnboarding = () => {
    setOnboardingCompleted(true);
  };

  const steps = [
    {
      title: "Welcome to PixelSense",
      description: "Your intelligent visual comfort companion. We analyze your screen and room lighting to reduce eye strain effortlessly.",
      icon: <Sparkles className="w-12 h-12 text-primary" />,
    },
    {
      title: "Environmental Awareness",
      description: "PixelSense uses your built-in ambient light sensor to detect exactly how bright your room is, matching your display to reality.",
      icon: <Sun className="w-12 h-12 text-amber-500" />,
    },
    {
      title: "Content Analysis",
      description: "We analyze the colors on your screen in real-time. If you switch from a dark IDE to a bright webpage, we gently compensate to protect your eyes.",
      icon: <Monitor className="w-12 h-12 text-blue-500" />,
    },
    {
      title: "100% Local & Private",
      description: "All analysis happens directly on your machine. We never record your screen, and we never send your data to the cloud. You are fully protected.",
      icon: <ShieldCheck className="w-12 h-12 text-green-500" />,
    }
  ];

  const current = steps[step];

  return (
    <div className="flex-1 flex items-center justify-center p-10 bg-background h-screen w-screen fixed inset-0 z-50">
      <motion.div 
        key={step}
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: -20 }}
        transition={{ duration: 0.5, ease: "easeOut" }}
        className="max-w-2xl w-full bg-card border border-border shadow-2xl rounded-2xl p-12 flex flex-col items-center text-center relative overflow-hidden"
      >
        <div className="absolute top-0 left-0 w-full h-1 bg-secondary">
          <motion.div 
            className="h-full bg-primary"
            initial={{ width: `${(step / steps.length) * 100}%` }}
            animate={{ width: `${((step + 1) / steps.length) * 100}%` }}
            transition={{ duration: 0.3 }}
          />
        </div>

        <div className="mb-8 p-6 bg-secondary/30 rounded-full">
          {current.icon}
        </div>
        
        <h1 className="text-3xl font-semibold tracking-tight text-foreground mb-4">
          {current.title}
        </h1>
        
        <p className="text-lg text-muted-foreground mb-12 max-w-lg">
          {current.description}
        </p>

        <div className="w-full flex items-center justify-between mt-auto">
          <button 
            onClick={completeOnboarding}
            className="text-muted-foreground hover:text-foreground font-medium px-4 py-2 transition-colors focus-visible:outline-ring rounded-md"
          >
            Skip Intro
          </button>
          
          {step < steps.length - 1 ? (
            <button 
              onClick={() => setStep(step + 1)}
              className="flex items-center gap-2 bg-primary hover:bg-primary/90 text-primary-foreground font-medium px-6 py-3 rounded-md transition-colors focus-visible:outline-ring"
            >
              Next <ArrowRight className="w-4 h-4" />
            </button>
          ) : (
            <button 
              onClick={completeOnboarding}
              className="flex items-center gap-2 bg-success hover:bg-success/90 text-success-foreground font-medium px-8 py-3 rounded-md transition-colors focus-visible:outline-ring"
            >
              Get Started <ArrowRight className="w-4 h-4" />
            </button>
          )}
        </div>
      </motion.div>
    </div>
  );
}
