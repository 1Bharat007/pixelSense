
interface LayoutProps {
  children: React.ReactNode;
  activeTab: string;
  onTabChange: (tabId: string) => void;
}

export const Layout: React.FC<LayoutProps> = ({ children, activeTab, onTabChange }) => {
  const tabs = [
    { id: 'overview', label: 'Overview' },
    { id: 'general', label: 'General' },
    { id: 'brightness', label: 'Brightness' },
    { id: 'adaptive', label: 'Adaptive' },
    { id: 'transition', label: 'Transition' },
    { id: 'performance', label: 'Performance' },
    { id: 'developer', label: 'Developer' },
    { id: 'diagnostics', label: 'Diagnostics' },
    { id: 'about', label: 'About' }
  ];

  return (
    <div className="flex min-h-screen w-full bg-background overflow-x-hidden">
      <nav className="w-64 border-r border-border bg-card/50 flex flex-col" aria-label="Main Navigation">
        <div className="p-6 font-semibold text-lg border-b border-border">PixelSense Settings</div>
        <div className="flex-1 overflow-y-auto py-4">
          {tabs.map(tab => (
            <button
              key={tab.id}
              className={`w-full text-left px-6 py-3 font-medium transition-colors ${
                activeTab === tab.id 
                  ? 'bg-primary/10 text-primary border-r-4 border-primary' 
                  : 'text-muted-foreground hover:bg-secondary/50 hover:text-foreground'
              }`}
              onClick={() => onTabChange(tab.id)}
              aria-current={activeTab === tab.id ? 'page' : undefined}
            >
              {tab.label}
            </button>
          ))}
        </div>
      </nav>
      <main className="flex-1 overflow-y-auto overflow-x-hidden relative" role="main">
        {children}
      </main>
    </div>
  );
};

