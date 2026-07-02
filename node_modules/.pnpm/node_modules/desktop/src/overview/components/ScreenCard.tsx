import React from 'react';
import { useOverview } from '../OverviewContext';

export const ScreenCard = () => {
  const { screen } = useOverview();
  return (
    <div className="card" tabIndex={0} aria-live="polite">
      <h2>Screen Content</h2>
      <div className="grid-2">
        <div className="stat-block">
          <span className="stat-label">Avg Luminance</span>
          <span className="stat-value">{screen.averageLuminance.toFixed(1)}</span>
        </div>
        <div className="stat-block">
          <span className="stat-label">Peak Luminance</span>
          <span className="stat-value">{screen.peakLuminance.toFixed(1)}</span>
        </div>
      </div>
    </div>
  );
};
