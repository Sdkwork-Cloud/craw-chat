import React, { useState } from "react";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';
import {
  PageLayout,
  Group,
  ListItem,
  ToggleItem,
} from "../../components/SettingsCommons";

export const FriendPermissions = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [phone, setPhone] = useState(true);
  const [wxid, setWxid] = useState(true);
  return (
    <PageLayout title={t("user:privacy.friend_perms", "Friend permissions")}>
      <Group>
        <ListItem
          label={t("user:privacy.blacklist", "Blacklist")}
          hideBorder
          onClick={() => navigate("/settings/friend-permissions/blacklist")}
        />
      </Group>
      <div className="px-4 py-2 text-[13px] text-text-sub">{t("user:privacy.add_methods", "Ways to add me")}</div>
      <Group>
        <ToggleItem label={t("user:privacy.phone", "Phone number")} checked={phone} onChange={setPhone} />
        <ToggleItem
          label={t("user:privacy.wechat_id", "WeChat ID")}
          checked={wxid}
          onChange={setWxid}
          hideBorder
        />
      </Group>
    </PageLayout>
  );
};

export const Privacy = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  return (
    <PageLayout title={t("user:privacy.info_perms", "Personal info & permissions")}>
      <Group>
        <ListItem
          label={t("user:privacy.sys_perms", "System permissions")}
          onClick={() => navigate("/settings/privacy/system")}
        />
        <ListItem
          label={t("user:privacy.auth_mgr", "Permission management")}
          hideBorder
          onClick={() => navigate("/settings/privacy/auth")}
        />
      </Group>
      <Group>
        <ListItem
          label={t("user:privacy.ads_mgr", "Personalized ad management")}
          hideBorder
          onClick={() => navigate("/settings/privacy/ads")}
        />
      </Group>
    </PageLayout>
  );
};

export const InfoCollection = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:privacy.collection_list", "Personal data collection list")}>
    <div className="p-4">
      <h3 className="text-[18px] font-bold text-text-main mb-4">{t("user:privacy.collection_list", "Personal data collection list")}</h3>
      <p className="text-[14px] text-text-sub mb-6 leading-relaxed">{t("user:privacy.collection_desc", "To provide you with Sdkwork IM H5 services, we need to collect the following personal information:")}</p>
      <Group>
        <ListItem label={t("user:privacy.basic_info", "Basic information")} rightText={t("user:privacy.basic_info_val", "Avatar, nickname, gender, region")} />
        <ListItem label={t("user:privacy.device_info", "Device information")} rightText={t("user:privacy.device_info_val", "Device model, operating system")} />
        <ListItem label={t("user:privacy.network_info", "Network information")} rightText={t("user:privacy.network_info_val", "IP address, network type")} />
        <ListItem label={t("user:privacy.log_info", "Log information")} rightText={t("user:privacy.log_info_val", "Operation logs, crash logs")} hideBorder />
      </Group>
    </div>
  </PageLayout>
  );
};

export const ThirdPartySharing = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:privacy.sharing_list", "Third-party data sharing list")}>
    <div className="p-4">
      <h3 className="text-[18px] font-bold text-text-main mb-4">{t("user:privacy.sharing_list", "Third-party data sharing list")}</h3>
      <p className="text-[14px] text-text-sub mb-6 leading-relaxed">{t("user:privacy.sharing_desc", "When providing services to you, we may share necessary information with the following third parties:")}</p>
      <Group>
        <ListItem label={t("user:privacy.map_provider", "Map service provider")} rightText={t("user:privacy.map_provider_val", "Location information")} />
        <ListItem label={t("user:privacy.push_provider", "Push service provider")} rightText={t("user:privacy.push_provider_val", "Device identifiers")} />
        <ListItem label={t("user:privacy.pay_provider", "Payment service provider")} rightText={t("user:privacy.pay_provider_val", "Order information")} hideBorder />
      </Group>
    </div>
  </PageLayout>
  );
};
