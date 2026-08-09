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
    <PageLayout title={t("user:privacy.friend_perms", "朋友权限")}>
      <Group>
        <ListItem
          label={t("user:privacy.blacklist", "通讯录黑名单")}
          hideBorder
          onClick={() => navigate("/settings/friend-permissions/blacklist")}
        />
      </Group>
      <div className="px-4 py-2 text-[13px] text-text-sub">{t("user:privacy.add_methods", "添加我的方式")}</div>
      <Group>
        <ToggleItem label={t("user:privacy.phone", "手机号")} checked={phone} onChange={setPhone} />
        <ToggleItem
          label={t("user:privacy.wechat_id", "微信号")}
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
    <PageLayout title={t("user:privacy.info_perms", "个人信息与权限")}>
      <Group>
        <ListItem
          label={t("user:privacy.sys_perms", "系统权限管理")}
          onClick={() => navigate("/settings/privacy/system")}
        />
        <ListItem
          label={t("user:privacy.auth_mgr", "授权管理")}
          hideBorder
          onClick={() => navigate("/settings/privacy/auth")}
        />
      </Group>
      <Group>
        <ListItem
          label={t("user:privacy.ads_mgr", "个性化广告管理")}
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
  <PageLayout title={t("user:privacy.collection_list", "个人信息收集清单")}>
    <div className="p-4">
      <h3 className="text-[18px] font-bold text-text-main mb-4">{t("user:privacy.collection_list", "个人信息收集清单")}</h3>
      <p className="text-[14px] text-text-sub mb-6 leading-relaxed">{t("user:privacy.collection_desc", "为了向您提供 ClawChat 的各项服务，我们需要收集您的以下个人信息：")}</p>
      <Group>
        <ListItem label={t("user:privacy.basic_info", "基本信息")} rightText={t("user:privacy.basic_info_val", "头像、昵称、性别、地区")} />
        <ListItem label={t("user:privacy.device_info", "设备信息")} rightText={t("user:privacy.device_info_val", "设备型号、操作系统")} />
        <ListItem label={t("user:privacy.network_info", "网络信息")} rightText={t("user:privacy.network_info_val", "IP地址、网络类型")} />
        <ListItem label={t("user:privacy.log_info", "日志信息")} rightText={t("user:privacy.log_info_val", "操作日志、崩溃日志")} hideBorder />
      </Group>
    </div>
  </PageLayout>
  );
};

export const ThirdPartySharing = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:privacy.sharing_list", "第三方信息共享清单")}>
    <div className="p-4">
      <h3 className="text-[18px] font-bold text-text-main mb-4">{t("user:privacy.sharing_list", "第三方信息共享清单")}</h3>
      <p className="text-[14px] text-text-sub mb-6 leading-relaxed">{t("user:privacy.sharing_desc", "在为您提供服务时，我们可能会与以下第三方共享您的必要信息：")}</p>
      <Group>
        <ListItem label={t("user:privacy.map_provider", "地图服务提供商")} rightText={t("user:privacy.map_provider_val", "位置信息")} />
        <ListItem label={t("user:privacy.push_provider", "推送服务提供商")} rightText={t("user:privacy.push_provider_val", "设备标识符")} />
        <ListItem label={t("user:privacy.pay_provider", "支付服务提供商")} rightText={t("user:privacy.pay_provider_val", "订单信息")} hideBorder />
      </Group>
    </div>
  </PageLayout>
  );
};
