import React from 'react';

/**
 * P2-22: Base skeleton shimmer block. Renders a gray placeholder with
 * a CSS shimmer animation (see `.im-h5-skeleton` in index.css).
 */
export const Skeleton: React.FC<{
  width?: string | number;
  height?: string | number;
  borderRadius?: string | number;
  className?: string;
}> = ({ width = '100%', height = 16, borderRadius = 4, className }) => (
  <div
    className={`im-h5-skeleton ${className ?? ''}`}
    style={{
      width: typeof width === 'number' ? `${width}px` : width,
      height: typeof height === 'number' ? `${height}px` : height,
      borderRadius: typeof borderRadius === 'number' ? `${borderRadius}px` : borderRadius,
    }}
  />
);

/**
 * P2-22: Skeleton placeholder for the inbox list. Mimics the layout of
 * `im-h5-chat-item` — a two-line card with title + preview on the left
 * and a timestamp on the right.
 */
export const InboxSkeleton: React.FC<{ count?: number }> = ({ count = 6 }) => (
  <div className="im-h5-skeleton-list" role="status" aria-label="Loading inbox">
    {Array.from({ length: count }, (_, index) => (
      <div key={index} className="im-h5-chat-item">
        <div className="im-h5-chat-item-main" style={{ flex: 1 }}>
          <Skeleton width="60%" height={18} />
          <div style={{ marginTop: 6 }}>
            <Skeleton width="85%" height={14} />
          </div>
        </div>
        <Skeleton width={36} height={12} />
      </div>
    ))}
  </div>
);

/**
 * P2-22: Skeleton placeholder for the message timeline. Mimics
 * `im-h5-chat-timeline-item` — a card with a meta row (sender + time)
 * and a body text line.
 */
export const TimelineSkeleton: React.FC<{ count?: number }> = ({ count = 5 }) => (
  <div className="im-h5-skeleton-list" role="status" aria-label="Loading messages">
    {Array.from({ length: count }, (_, index) => (
      <div key={index} className="im-h5-chat-timeline-item">
        <div className="im-h5-chat-timeline-meta">
          <Skeleton width="30%" height={12} />
          <Skeleton width={48} height={12} />
        </div>
        <div style={{ marginTop: 6 }}>
          <Skeleton width="90%" height={14} />
        </div>
      </div>
    ))}
  </div>
);

/**
 * P2-22: Slim loading bar shown at the top of the timeline when fetching
 * older messages. Replaces the text "Loading earlier messages…" to avoid
 * layout shift.
 */
export const TimelineLoadingMoreBar: React.FC = () => (
  <div
    role="status"
    aria-label="Loading earlier messages"
    style={{ display: 'flex', justifyContent: 'center', padding: '8px 0' }}
  >
    <Skeleton width={120} height={12} />
  </div>
);
