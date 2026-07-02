import { Users, Briefcase, MoonStar, Gamepad2, Palette } from "lucide-react";
import { Card } from "../components/ui/Card";
import { useStore } from "../store/useStore";

const PROFILES = [
  { id: "Productivity", icon: Briefcase, desc: "Balanced comfort for reading and writing." },
  { id: "Night Owl", icon: MoonStar, desc: "Aggressive dimming and blue light reduction." },
  { id: "Gaming", icon: Gamepad2, desc: "Suspends adaptations for maximum frame rates." },
  { id: "Color Critical", icon: Palette, desc: "Static curve for accurate color grading." },
];

export function Profiles() {
  const { dashboard } = useStore();
  const activeProfile = dashboard?.comfort.active_profile || "Productivity";

  return (
    <div className="flex-1 p-10 overflow-y-auto w-full max-w-[1600px] mx-auto">
      <header className="mb-10">
        <h1 className="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
          <Users className="w-8 h-8 text-accent" />
          Comfort Profiles
        </h1>
        <p className="text-muted-foreground mt-1">Select a tuning profile for the intelligence engine.</p>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-4xl">
        {PROFILES.map((profile) => {
          const Icon = profile.icon;
          const isActive = activeProfile === profile.id;
          
          return (
            <Card 
              key={profile.id} 
              className={`cursor-pointer transition-all duration-200 hover:border-accent ${
                isActive ? "border-accent ring-1 ring-accent bg-accent/5" : ""
              }`}
            >
              <div className="flex items-start gap-4">
                <div className={`p-3 rounded-lg ${isActive ? "bg-accent text-accent-foreground" : "bg-secondary text-muted-foreground"}`}>
                  <Icon className="w-6 h-6" />
                </div>
                <div>
                  <h3 className="text-lg font-medium text-foreground">{profile.id}</h3>
                  <p className="text-sm text-muted-foreground mt-1">{profile.desc}</p>
                </div>
              </div>
              
              {isActive && (
                <div className="absolute top-4 right-4 flex h-3 w-3">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-accent opacity-75"></span>
                  <span className="relative inline-flex rounded-full h-3 w-3 bg-accent"></span>
                </div>
              )}
            </Card>
          )
        })}
      </div>
    </div>
  );
}
