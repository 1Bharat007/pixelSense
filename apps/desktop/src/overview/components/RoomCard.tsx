import React from 'react';
import { useOverview } from '../OverviewContext';

export const RoomCard = () => {
  const { ambient } = useOverview();
  return (
    <div className="card" tabIndex={0} aria-live="polite">
      <h2>Room (Ambient)</h2>
      <div className="grid-2">
        <div className="stat-block">
          <span className="stat-label">Current Lux</span>
          <span className="stat-value">{ambient.currentLux.toFixed(1)}</span>
        </div>
        <div className="stat-block">
          <span className="stat-label">Environment</span>
          <span className="stat-value">{ambient.environment}</span>
        </div>
      </div>
    </div>
  );
};
