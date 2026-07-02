import React from 'react';
import { useOverview } from '../OverviewContext';
import { StatusBadge } from './StatusBadge';

export const HeroCard = () => {
  const { hero } = useOverview();
  
  return (
    <div className="card hero-card" tabIndex={0}>
      <div className="hero-header">
        <h2>Current Comfort</h2>
        <StatusBadge status={hero.comfortStatus} />
      </div>
      <div className="hero-content">
        <div className="hero-stat">
          <span className="stat-label">Recommendation</span>
          <span className="stat-value">{hero.recommendation}</span>
        </div>
        <div className="hero-stat">
          <span className="stat-label">Reason</span>
          <span className="stat-value text-small">{hero.reason}</span>
        </div>
        <div className="hero-stat">
          <span className="stat-label">Confidence</span>
          <span className="stat-value">{(hero.confidence * 100).toFixed(0)}%</span>
        </div>
      </div>
    </div>
  );
};
