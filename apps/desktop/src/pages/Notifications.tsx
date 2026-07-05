import { useState, useEffect } from "react";
import { NotificationEvent, NotificationService } from "../services/notifications";
import { Card } from "../components/ui/Card";
import { Bell, Search, Info, AlertTriangle, ShieldAlert, Check } from "lucide-react";
import { FixedSizeList as List } from "react-window";

export function Notifications() {
  const [events, setEvents] = useState<NotificationEvent[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState("");

  useEffect(() => {
    NotificationService.getNotifications().then(data => {
      setEvents(data);
      setLoading(false);
    });
  }, []);

  const filtered = events.filter(e => 
    e.title.toLowerCase().includes(search.toLowerCase()) || 
    e.message.toLowerCase().includes(search.toLowerCase())
  );

  const markAllRead = () => {
    setEvents(events.map(e => ({ ...e, read: true })));
  };

  const getPriorityIcon = (prio: string) => {
    switch (prio) {
      case "High": return <ShieldAlert className="w-5 h-5 text-destructive" />;
      case "Normal": return <Info className="w-5 h-5 text-primary" />;
      default: return <Bell className="w-5 h-5 text-muted-foreground" />;
    }
  };

  const Row = ({ index, style }: { index: number; style: React.CSSProperties }) => {
    const event = filtered[index];
    const date = new Date(event.timestamp);
    
    return (
      <div style={style} className="px-1 py-1">
        <Card className={`flex flex-col md:flex-row items-start md:items-center justify-between p-5 transition-shadow shadow-sm ${!event.read ? 'border-l-4 border-l-primary bg-secondary/10' : 'bg-card border-border'}`}>
          <div className="flex gap-4 items-start">
            <div className="mt-1">
              {getPriorityIcon(event.priority)}
            </div>
            <div>
              <h3 className={`text-base ${!event.read ? 'font-semibold text-foreground' : 'font-medium text-foreground/80'}`}>
                {event.title}
              </h3>
              <p className="text-sm text-muted-foreground mt-1 max-w-xl leading-relaxed">{event.message}</p>
              <div className="text-xs text-muted-foreground/60 mt-2 font-medium tracking-wide">
                {date.toLocaleTimeString()} · {date.toLocaleDateString()}
              </div>
            </div>
          </div>
          
          {event.action_type && (
            <div className="mt-4 md:mt-0 md:ml-6 shrink-0">
              <button className="px-4 py-2 bg-primary hover:bg-primary/90 text-primary-foreground font-medium rounded-md transition-colors text-sm focus-visible:outline-ring">
                {event.action_type}
              </button>
            </div>
          )}
        </Card>
      </div>
    );
  };

  return (
    <div className="flex-1 p-10 overflow-y-hidden w-full max-w-5xl mx-auto h-full flex flex-col">
      <header className="mb-6 flex flex-col gap-4 md:flex-row md:items-center justify-between shrink-0">
        <div>
          <h1 className="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
            <Bell className="w-8 h-8 text-primary" />
            Notification Center
          </h1>
          <p className="text-muted-foreground mt-1">Review system alerts and intelligent recommendations.</p>
        </div>
        
        <div className="flex gap-3 items-center">
          <div className="relative">
            <Search className="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground" />
            <input 
              type="text"
              placeholder="Search alerts..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="pl-9 pr-4 py-2 bg-input/50 border border-border rounded-md text-sm outline-none focus:ring-2 focus:ring-primary w-64 text-foreground"
            />
          </div>
          <button 
            onClick={markAllRead}
            className="flex items-center gap-2 px-4 py-2 bg-secondary hover:bg-secondary/80 text-foreground font-medium rounded-md transition-colors text-sm"
          >
            <Check className="w-4 h-4" /> Dismiss All
          </button>
        </div>
      </header>

      <div className="flex-1 min-h-0 bg-background/50 rounded-xl border border-border/50 relative">
        {loading ? (
          <div className="absolute inset-0 flex items-center justify-center">
             <div className="animate-pulse flex flex-col gap-4 w-full px-6">
              {[1,2,3,4].map(i => <div key={i} className="h-24 bg-secondary/30 rounded-xl w-full" />)}
            </div>
          </div>
        ) : filtered.length === 0 ? (
          <div className="absolute inset-0 flex flex-col items-center justify-center text-muted-foreground p-8 text-center">
             <div className="w-16 h-16 rounded-full bg-secondary flex items-center justify-center mb-4">
               <Bell className="w-8 h-8 opacity-50" />
             </div>
             <h3 className="text-lg font-medium text-foreground mb-1">You're all caught up!</h3>
             <p className="text-sm">No notifications match your current filters.</p>
          </div>
        ) : (
          <List
            height={600}
            itemCount={filtered.length}
            itemSize={120}
            width="100%"
            className="custom-scrollbar"
            style={{ height: '100%' }}
          >
            {Row}
          </List>
        )}
      </div>
    </div>
  );
}
