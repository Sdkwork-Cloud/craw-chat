import React from 'react';

export const MeetingFormItem = ({
  label,
  children,
  required = false,
  onClick,
}: {
  label: string;
  children?: React.ReactNode;
  required?: boolean;
  onClick?: () => void;
}) => (
  <div
    className="flex items-center px-4 py-3 border-b border-border-color/30 last:border-b-0 bg-white dark:bg-[#1a1b1c] active:bg-gray-50 dark:active:bg-[#202122] transition-colors"
    onClick={onClick}
  >
    <div className="w-[80px] shrink-0 text-[15px] text-text-main flex items-center">
      {required && <span className="text-rose-500 mr-1">*</span>}
      {label}
    </div>
    <div className="flex-1 flex items-center min-w-0">{children}</div>
  </div>
);
