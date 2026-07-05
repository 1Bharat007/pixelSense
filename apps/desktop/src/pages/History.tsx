import { useState, useEffect } from "react";
import { type HistoryEvent, HistoryService } from "../services/history";
import { Card } from "../components/ui/Card";
import { Search, Filter, Calendar, Activity, Sun, CheckCircle, SlidersHorizontal, Settings2, User } from "lucide-react";

export function History() {
  const [events, setEvents] = useState<HistoryEvent[]>([]);
  const [loading, setLoading] = useState(true);
  
  const [search, setSearch] = useState("");
  const [filterCategory, setFilterCategory] = useState<string>("All");

  useEffect(() => {
    HistoryService.getHistory().then(data => {
      setEvents(data);
      setLoading(false);
    });
  }, []);

  const filtered = events.filter(e => {
    const matchesSearch = e.description.toLowerCase().includes(search.toLowerCase()) || e.category.toLowerCase().includes(search.toLowerCase());
    const matchesCategory = filterCategory === "All" || e.category === filterCategory;
    return matchesSearch && matchesCategory;
  });

  const getIconForCategory = (cat: string) => {
    switch (cat) {
      case "Brightness": return <Sun className="w-4 h-4" />;
      case "Comfort": return <Activity className="w-4 h-4" />;
      case "Profile": return <User className="w-4 h-4" />;
      case "System": return <Settings2 className="w-4 h-4" />;
      default: return <CheckCircle className="w-4 h-4" />;
    }
  };

  const Row = ({ index, style }: { index: number; style: React.CSSProperties }) => {
    const event = filtered[index];
    const date = new Date(event.timestamp);
    
    return (
      <div style={style} className="px-1 py-1">
        <Card className="flex flex-col md:flex-row items-start md:items-center justify-between p-4 bg-card border-border shadow-sm hover:shadow-md transition-shadow">
          <div className="flex gap-4 items-center">
            <div className="p-3 bg-secondary rounded-full text-foreground/70">
              {getIconForCategory(event.category)}
            </div>
            <div>
              <h3 className="font-medium text-foreground">{event.category}</h3>
              <p className="text-sm text-muted-foreground">{event.description}</p>
            </div>
          </div>
          
          <div className="mt-4 md:mt-0 flex flex-col md:items-end text-sm text-muted-foreground gap-1">
            <div className="flex items-center gap-1">
              <Calendar className="w-3 h-3" />
              <span>{date.toLocaleDateString()} {date.toLocaleTimeString()}</span>
            </div>
            {event.before_value && event.after_value && (
              <div className="flex items-center gap-2 font-mono text-xs bg-secondary px-2 py-1 rounded-md text-foreground">
                <span className="line-through opacity-60">{event.before_value}</span>
                <span>→</span>
                <span className="font-semibold text-primary">{event.after_value}</span>
              </div>
            )}
          </div>
        </Card>
      </div>
    );
  };

  return (
    <div className="flex-1 p-10 overflow-y-hidden w-full max-w-6xl mx-auto h-full flex flex-col">
      <header className="mb-6 flex flex-col gap-4 md:flex-row md:items-center justify-between shrink-0">
        <div>
          <h1 className="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
            <SlidersHorizontal className="w-8 h-8 text-primary" />
            History Log
          </h1>
          <p className="text-muted-foreground mt-1">Review system events, brightness changes, and adaptations.</p>
        </div>
        
        <div className="flex gap-3 items-center">
          <div className="relative">
            <Search className="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground" />
            <input 
              type="text"
              placeholder="Search history..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="pl-9 pr-4 py-2 bg-input/50 border border-border rounded-md text-sm outline-none focus:ring-2 focus:ring-primary w-64 text-foreground"
            />
          </div>
          <div className="relative">
            <Filter className="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
            <select 
              value={filterCategory}
              onChange={(e) => setFilterCategory(e.target.value)}
              className="pl-9 pr-4 py-2 bg-input/50 border border-border rounded-md text-sm outline-none focus:ring-2 focus:ring-primary appearance-none cursor-pointer text-foreground min-w-[140px]"
            >
              <option value="All">All Categories</option>
              <option value="Brightness">Brightness</option>
              <option value="Comfort">Comfort</option>
              <option value="Profile">Profile</option>
              <option value="System">System</option>
            </select>
          </div>
        </div>
      </header>

      {/* Analytics Summary */}
      <section className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6 shrink-0">
        <Card className="p-4 flex flex-col justify-between">
          <h3 className="text-sm font-medium text-muted-foreground flex items-center gap-2">
            <Activity className="w-4 h-4" /> Average Comfort
          </h3>
          <p className="text-2xl font-semibold text-foreground mt-2">9.2 <span className="text-sm font-normal text-success">↑ optimal</span></p>
        </Card>
        <Card className="p-4 flex flex-col justify-between">
          <h3 className="text-sm font-medium text-muted-foreground flex items-center gap-2">
            <Sun className="w-4 h-4" /> Avg Room Lux
          </h3>
          <p className="text-2xl font-semibold text-foreground mt-2">240 <span className="text-sm font-normal text-muted-foreground">indoor</span></p>
        </Card>
        <Card className="p-4 flex flex-col justify-between">
          <h3 className="text-sm font-medium text-muted-foreground flex items-center gap-2">
            <CheckCircle className="w-4 h-4" /> Auto-Adjustments
          </h3>
          <p className="text-2xl font-semibold text-foreground mt-2">{filtered.length} <span className="text-sm font-normal text-muted-foreground">this week</span></p>
        </Card>
        <Card className="p-4 flex flex-col justify-between">
          <h3 className="text-sm font-medium text-muted-foreground flex items-center gap-2">
            <User className="w-4 h-4" /> Manual Overrides
          </h3>
          <p className="text-2xl font-semibold text-foreground mt-2">0 <span className="text-sm font-normal text-success">perfectly tuned</span></p>
        </Card>
      </section>

      <div className="flex-1 min-h-0 bg-background/50 rounded-xl border border-border/50 relative">
        {loading ? (
          <div className="absolute inset-0 flex items-center justify-center">
            <div className="animate-pulse flex flex-col gap-4 w-full px-6">
              {[1,2,3,4].map(i => <div key={i} className="h-20 bg-secondary/30 rounded-xl w-full" />)}
            </div>
          </div>
        ) : filtered.length === 0 ? (
          <div className="absolute inset-0 flex flex-col items-center justify-center text-muted-foreground p-8 text-center">
             <div className="w-16 h-16 rounded-full bg-secondary flex items-center justify-center mb-4">
               <Search className="w-8 h-8 opacity-50" />
             </div>
             <h3 className="text-lg font-medium text-foreground mb-1">No events found</h3>
             <p className="text-sm">Try adjusting your filters or search query.</p>
          </div>
        ) : (
          <div className="h-full overflow-y-auto custom-scrollbar p-2 space-y-2">
            {filtered.map((event, index) => (
              <Row key={event.id} index={index} style={{}} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
