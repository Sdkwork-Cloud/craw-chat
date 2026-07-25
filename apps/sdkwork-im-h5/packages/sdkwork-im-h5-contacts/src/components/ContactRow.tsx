import React from 'react';
import { cn } from '@sdkwork/im-h5-commons';
import { useNavigate } from 'react-router';
import { type Contact } from '../services/ContactService';

export const ContactRow: React.FC<{
  contact: Contact;
  isLast: boolean;
}> = ({ contact, isLast }) => {
  const navigate = useNavigate();
  return (
    <div
      className="flex items-center pl-4 pr-3 bg-bg-color active:bg-active-bg transition-colors cursor-pointer"
      onClick={() => navigate(`/chat/${contact.id}/profile`)}
    >
      <div className="w-10 h-10 rounded-md overflow-hidden shrink-0 mr-3.5 my-2">
        <img
          src={contact.avatar}
          alt={contact.name}
          className="w-full h-full object-cover"
        />
      </div>
      <div
        className={cn(
          "flex-1 min-h-[56px] flex items-center",
          !isLast && "border-b border-border-color/50",
        )}
      >
        <span className="text-[16px] text-text-main">{contact.name}</span>
      </div>
    </div>
  );
};
