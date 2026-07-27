import React from "react";
import { useNavigate } from "react-router";
import { useAppStore } from "@sdkwork/im-h5-core";

import { ProfileHeaderCard } from "../components/ProfileHeaderCard";
import { MeHeader } from "../components/MeHeader";
import { MeServicesSection } from "../components/me/MeServicesSection";
import { MeAssetsSection } from "../components/me/MeAssetsSection";
import { MeFeaturesSection } from "../components/me/MeFeaturesSection";
import { MeSettingsSection } from "../components/me/MeSettingsSection";

export const Me: React.FC = () => {
  const { currentUser } = useAppStore();
  const navigate = useNavigate();

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-[#0a0a0a] overflow-y-auto pb-[84px]">
      {/* Header */}
      <MeHeader onContactClick={() => navigate('/workspace/contacts')} />

      <div className="flex flex-col mt-2">
        {/* Profile Section */}
        <ProfileHeaderCard
          currentUser={currentUser}
          onClick={() => navigate("/my-profile")}
        />

        {/* Services */}
        <MeServicesSection />

        {/* AI Assets */}
        <MeAssetsSection />

        {/* Features */}
        <MeFeaturesSection />

        {/* Settings */}
        <MeSettingsSection />
      </div>
    </div>
  );
};

