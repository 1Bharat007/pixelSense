import React from 'react';
import { useOverview } from '../OverviewContext';

export const DisplayCard = () => {
  const { display } = useOverview();
  return (
    <div className="card" tabIndex={0} aria-live="polite">
      <h2>Display Output</h2>
      <div className="grid-2">
        <div className="stat-block">
          <span className="stat-label">Hardware Brightness</span>
          <span className="stat-value">{display.currentBrightness}%</span>
        </div>
        <div className="stat-block">
          <span className="stat-label">Target Brightness</span>
          <span className="stat-value">{display.recommendedBrightness}%</span>
        </div>
      </div>
    </div>
  );
};
