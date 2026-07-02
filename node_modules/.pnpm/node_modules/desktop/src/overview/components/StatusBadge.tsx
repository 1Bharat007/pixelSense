import React from 'react';

type StatusType = 'Comfortable' | 'Adjusting' | 'Attention' | 'Disabled' | 'Healthy' | 'Degraded' | 'Offline' | 'Active';

export const StatusBadge: React.FC<{ status: StatusType | string }> = ({ status }) => {
  let icon = '?';
  let colorClass = 'status-default';

  switch (status) {
    case 'Comfortable':
    case 'Healthy':
    case 'Active':
      icon = '??';
      colorClass = 'status-good';
      break;
    case 'Adjusting':
    case 'Degraded':
      icon = '??';
      colorClass = 'status-warn';
      break;
    case 'Attention':
    case 'Offline':
      icon = '??';
      colorClass = 'status-error';
      break;
    case 'Disabled':
      icon = '?';
      colorClass = 'status-disabled';
      break;
  }

  return (
    <span className={`status-badge ${colorClass}`} aria-label={`Status: ${status}`}>
      {icon} {status}
    </span>
  );
};
