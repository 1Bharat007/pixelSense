const fs = require('fs');

const history = [];
const notifications = [];

const now = Date.now();

// Generate 100 history events over the last 3 days
for (let i = 0; i < 100; i++) {
  const ts = now - (Math.random() * 3 * 24 * 60 * 60 * 1000);
  
  const categories = ["Brightness", "Comfort", "Profile", "System"];
  const cat = categories[Math.floor(Math.random() * categories.length)];
  
  let desc = "";
  let b = null;
  let a = null;
  
  if (cat === "Brightness") {
    desc = "Automatically adjusted brightness due to room light change.";
    b = (Math.floor(Math.random() * 50) + 20).toString();
    a = (Math.floor(Math.random() * 50) + 40).toString();
  } else if (cat === "Profile") {
    desc = "User manually switched active comfort profile.";
    b = "Reading";
    a = "Productivity";
  } else if (cat === "Comfort") {
    desc = "Comfort score dropped below optimal threshold.";
    b = "9.5";
    a = "7.2";
  } else {
    desc = "Background worker initialized successfully.";
  }

  history.push({
    id: `hist_${i}`,
    timestamp: Math.floor(ts),
    category: cat,
    description: desc,
    before_value: b,
    after_value: a
  });
}

// Generate 15 notifications
for (let i = 0; i < 15; i++) {
  const ts = now - (Math.random() * 24 * 60 * 60 * 1000);
  const priorities = ["Low", "Normal", "High"];
  const prio = priorities[Math.floor(Math.random() * priorities.length)];
  
  notifications.push({
    id: `notif_${i}`,
    timestamp: Math.floor(ts),
    priority: prio,
    title: prio === "High" ? "Sensor Disconnected" : "Brightness Optimized",
    message: prio === "High" ? "The ambient light sensor stopped responding." : "We reduced your brightness by 15% to match your room.",
    read: Math.random() > 0.5,
    action_type: prio === "High" ? "Retry" : null
  });
}

history.sort((x, y) => y.timestamp - x.timestamp);
notifications.sort((x, y) => y.timestamp - x.timestamp);

fs.writeFileSync('src-tauri/history.jsonl', history.map(h => JSON.stringify(h)).join('\n'));
fs.writeFileSync('src-tauri/notifications.jsonl', notifications.map(n => JSON.stringify(n)).join('\n'));
console.log("Seeded history.jsonl and notifications.jsonl");
