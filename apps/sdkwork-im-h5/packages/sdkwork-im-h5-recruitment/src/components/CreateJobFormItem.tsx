import React from 'react';

export const CreateJobFormItem: React.FC<{
  icon: React.ReactNode;
  value: string;
  onChange?: (val: string) => void;
  placeholder?: string;
  onClick?: () => void;
  label?: string;
  isSelect?: boolean;
}> = ({ icon, value, onChange, placeholder, onClick, label, isSelect }) => {
  return (
    <div
      className={`flex items-center px-4 py-3.5 border-b border-border-color/30 last:border-b-0 ${
        isSelect ? 'cursor-pointer active:bg-bg-color justify-between' : ''
      }`}
      onClick={isSelect ? onClick : undefined}
    >
      <div className="flex items-center flex-1">
        <div className="text-text-sub mr-3 shrink-0">
          {icon}
        </div>
        {isSelect ? (
          <span className="text-[16px] text-text-main font-medium">{label}</span>
        ) : (
          <input
            type="text"
            className="flex-1 bg-transparent border-none outline-none text-[16px] text-text-main placeholder:text-text-sub/50"
            placeholder={placeholder}
            value={value}
            onChange={(e) => onChange?.(e.target.value)}
          />
        )}
      </div>
      {isSelect && <span className="text-text-sub">{value} &gt;</span>}
    </div>
  );
};
