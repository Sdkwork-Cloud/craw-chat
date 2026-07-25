import React from 'react';
import { Avatar } from '@sdkwork/im-h5-commons';
import { Briefcase, GraduationCap } from 'lucide-react';

export const CandidateHeader: React.FC<{ candidate: any }> = ({ candidate }) => {
  return (
    <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-5 mb-4 shadow-sm border border-border-color/30 flex items-start gap-4">
      <Avatar
        src={candidate.avatar}
        fallback={candidate.name.substring(0, 1)}
        size="xl"
      />
      <div className="flex-1">
        <h2 className="text-xl font-bold text-text-main mb-1">
          {candidate.name}
        </h2>
        <div className="text-[15px] text-text-sub font-medium mb-2">
          {candidate.jobTitle}
        </div>
        <div className="flex flex-wrap gap-2">
          <span className="bg-bg-color px-2 py-1 rounded text-xs text-text-sub flex items-center gap-1">
            <Briefcase className="w-3 h-3" /> {candidate.experience}
          </span>
          <span className="bg-bg-color px-2 py-1 rounded text-xs text-text-sub flex items-center gap-1">
            <GraduationCap className="w-3 h-3" /> {candidate.education}
          </span>
        </div>
      </div>
    </div>
  );
};
