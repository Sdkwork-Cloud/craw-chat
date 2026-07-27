import React from 'react';
import { motion } from 'motion/react';
import { Avatar } from '@sdkwork/im-h5-commons';
import { Briefcase, Clock } from 'lucide-react';
import { useNavigate } from 'react-router';
import { CandidateRecord } from '../services/RecruitmentService';
import { useTranslation } from 'react-i18next';

export const CandidateItemCard: React.FC<{ candidate: CandidateRecord }> = ({ candidate }) => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  

  return (
    <motion.div
      whileTap={{ scale: 0.98 }}
      onClick={() => navigate(`/workspace/recruitment/${candidate.id}`)}
      className="bg-white dark:bg-[#2c2d2e] p-4 rounded-xl cursor-pointer shadow-sm border border-border-color/30"
    >
      <div className="flex justify-between items-start mb-3">
        <div className="flex items-center gap-3">
          <Avatar
            src={candidate.avatar}
            fallback={candidate.name.charAt(0)}
            size="md"
          />
          <div>
            <div className="text-[16px] font-medium text-text-main leading-tight mb-1">
              {candidate.name}
            </div>
            <div className="text-[13px] text-text-sub flex items-center gap-2">
              <span>{candidate.experience}</span>
              <span className="w-1 h-1 bg-border-color rounded-full" />
              <span>{candidate.education}</span>
            </div>
          </div>
        </div>
        <div className="flex flex-col items-end">
          <span className="text-[14px] font-medium text-primary-blue mb-1">
            {candidate.stage}
          </span>
        </div>
      </div>

      <div className="text-[14px] text-text-main bg-[#f8f9fa] dark:bg-[#202122] p-3 rounded-lg flex flex-col gap-2">
        <div className="flex items-center gap-2 text-[13px]">
          <Briefcase className="w-4 h-4 text-text-sub" />
          <span>{t('recruitment.apply', { job: candidate.jobTitle })}</span>
        </div>
        <div className="flex items-center gap-2 text-[13px]">
          <Clock className="w-4 h-4 text-text-sub" />
          <span className="text-orange-600 dark:text-orange-400">
            {candidate.date}
          </span>
        </div>
      </div>
    </motion.div>
  );
};
