import React, { useState } from "react";
import {
  showPrompt,
  PageLayout,
  showToast,
} from "@sdkwork/im-h5-commons";
import {
  Briefcase,
  Building,
  MapPin,
  DollarSign,
  GraduationCap,
  Clock,
} from "lucide-react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { CreateJobFormItem } from "../components/CreateJobFormItem";

export const CreateJob = () => {
  const { t } = useTranslation();

  
const navigate = useNavigate();
  
  const [formData, setFormData] = useState({
    title: "",
    department: "",
    location: "深圳",
    salary: "",
    experience: t('recruitment.createJob.unlimited'),
    education: t('recruitment.createJob.bachelor'),
  });

  const handleSubmit = () => {
  if (!formData.title) return showToast(t('recruitment.createJob.jobTitlePlaceholder'));
    if (!formData.department) return showToast(t('recruitment.createJob.departmentPlaceholder'));

    showToast(t('common.success', { defaultValue: '成功' }));
    navigate(-1);
  };

  return (
    <PageLayout
      title={t('recruitment.createJob.title')}
      rightElement={
        <span
          className="text-[16px] text-accent-blue font-medium active:opacity-60 cursor-pointer"
          onClick={handleSubmit}
        >{t('common.publish', { defaultValue: '发布' })}</span>
      }
    >
      <div className="p-4 space-y-4">
        {/* Basic Info */}
        <div className="bg-chat-other-bg rounded-xl overflow-hidden shadow-sm border border-border-color/30">
          <CreateJobFormItem
            icon={<Briefcase className="w-5 h-5" />}
            value={formData.title}
            onChange={(val) => setFormData({ ...formData, title: val })}
            placeholder={t('recruitment.createJob.jobTitlePlaceholder')}
          />
          <CreateJobFormItem
            icon={<Building className="w-5 h-5" />}
            value={formData.department}
            onChange={(val) => setFormData({ ...formData, department: val })}
            placeholder={t('recruitment.createJob.departmentPlaceholder')}
          />
          <CreateJobFormItem
            icon={<MapPin className="w-5 h-5" />}
            value={formData.location}
            onChange={(val) => setFormData({ ...formData, location: val })}
            placeholder={t('recruitment.createJob.locationPlaceholder')}
          />
          <CreateJobFormItem
            icon={<DollarSign className="w-5 h-5" />}
            value={formData.salary}
            onChange={(val) => setFormData({ ...formData, salary: val })}
            placeholder={t('recruitment.createJob.salaryPlaceholder')}
          />
        </div>

        {/* Requirements */}
        <div className="bg-chat-other-bg rounded-xl overflow-hidden shadow-sm border border-border-color/30">
          <CreateJobFormItem
            isSelect
            icon={<Clock className="w-5 h-5" />}
            label={t('recruitment.createJob.experienceLabel')}
            value={formData.experience}
            onClick={async () => {
              const exp = await showPrompt(t('recruitment.createJob.experiencePrompt'), formData.experience);
              if (exp) setFormData({ ...formData, experience: exp });
            }}
          />
          <CreateJobFormItem
            isSelect
            icon={<GraduationCap className="w-5 h-5" />}
            label={t('recruitment.createJob.educationLabel')}
            value={formData.education}
            onClick={async () => {
              const edu = await showPrompt(t('recruitment.createJob.educationPrompt'), formData.education);
              if (edu) setFormData({ ...formData, education: edu });
            }}
          />
        </div>

        {/* Job Description */}
        <div className="bg-chat-other-bg rounded-xl p-4 shadow-sm border border-border-color/30">
          <textarea
            className="w-full bg-transparent border-none outline-none text-[15px] text-text-main placeholder:text-text-sub/50 resize-none h-40"
            placeholder={t('recruitment.createJob.requirementPlaceholder')}
          />
        </div>
      </div>
    </PageLayout>
  );
};
