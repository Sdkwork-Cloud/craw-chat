import React, { useState } from "react";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';
import { Monitor, Smartphone, Laptop, Tablet } from "lucide-react";
import {
  showToast,
  showPrompt,
  Avatar,
  cn,
} from "@sdkwork/im-h5-commons";
import { PageLayout, Group, ListItem } from "../../components/SettingsCommons";

export const AccountSecurity = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  return (
    <PageLayout title={t("user:settings.account_security", "账号与安全")}>
      <Group>
        <ListItem
          label={t('user.auto_prop_1712c64', '微信号')}
          rightText="wxid_123456789"
          onClick={() => navigate("/settings/account/wechat-id")}
        />
        <ListItem
          label={t("user:account_sec.phone", "手机号")}
          rightText="138****8888"
          hideBorder
          onClick={() => navigate("/settings/account/phone")}
        />
      </Group>
      <Group>
        <ListItem
          label={t('user.auto_prop_2cb5ca2e', '微信密码')}
          onClick={() => navigate("/settings/account/password")}
        />
        <ListItem
          label={t("user:account_sec.voice_lock", "声音锁")}
          rightText={t("user:account_sec.not_set", "未设置")}
          hideBorder
          onClick={() => navigate("/settings/account/voice-lock")}
        />
      </Group>
      <Group>
        <ListItem
          label={t("user:account_sec.emergency_contacts", "应急联系人")}
          hideBorder
          onClick={() => navigate("/settings/account/emergency")}
        />
      </Group>
      <Group>
        <ListItem
          label={t('user.auto_prop_n51f86178', '登录设备管理')}
          hideBorder
          onClick={() => navigate("/settings/devices")}
        />
      </Group>
      <Group>
        <ListItem
          label={t('user.auto_prop_n3ed8e3ab', '更多安全设置')}
          hideBorder
          onClick={() => navigate("/settings/account/more")}
        />
      </Group>
    </PageLayout>
  );
};

export const Devices = () => {
  const { t } = useTranslation();
  
const [devices, setDevices] = useState([
    { id: 1, type: "laptop", name: "MacBook Pro", lastActive: "昨天 10:23" },
    { id: 2, type: "tablet", name: "iPad Pro 11", lastActive: "5月2日 14:00" },
  ]);

  const handleLogout = (id: number) => {
  setDevices(devices.filter((d) => d.id !== id));
    showToast(t('user.auto_fn_n28d92598', '已下线该设备'));
  };

  return (
    <PageLayout title={t('user.auto_prop_114509', '设备')}>
      <div className="px-4 py-8 flex flex-col items-center border-b border-border-color bg-chat-other-bg">
        <div className="w-20 h-20 mb-4 bg-[#00B42A] rounded-full flex items-center justify-center shadow-lg shadow-[#00B42A]/20">
          <Monitor className="w-10 h-10 text-white" />
        </div>
        <h2 className="text-xl font-bold text-text-main mb-1">{t('user.auto_n24dcb9e', '多设备登录管理')}</h2>
        <p className="text-[14px] text-text-sub">{t('user.auto_n4a4d7fcc', '你可以同时在多个设备上登录 Sdkwork IM H5')}</p>
      </div>

      <div className="p-4">
        <h3 className="text-[13px] text-text-sub mb-2 ml-1">{t('user.auto_2c9b33e3', '当前设备')}</h3>
        <div className="bg-chat-other-bg rounded-xl overflow-hidden mb-6">
          <div className="flex items-center p-4">
            <div className="w-12 h-12 bg-bg-color rounded-full flex items-center justify-center mr-4">
              <Smartphone className="w-6 h-6 text-text-main" />
            </div>
            <div className="flex-1">
              <h4 className="text-[16px] font-medium text-text-main">
                iPhone 15 Pro
              </h4>
              <p className="text-[13px] text-primary-blue mt-0.5">{t('user.auto_n5d6a62e2', 'Sdkwork IM H5 1.0.0 (在线)')}</p>
            </div>
          </div>
        </div>

        {devices.length > 0 && (
          <>
            <h3 className="text-[13px] text-text-sub mb-2 ml-1">{t('user.auto_n1ae2f0ec', '最近登录设备')}</h3>
            <div className="bg-chat-other-bg rounded-xl overflow-hidden">
              {devices.map((d, i) => (
                <div
                  key={d.id}
                  className={cn(
                    "flex items-center justify-between p-4 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer",
                    i !== devices.length - 1 && "border-b border-border-color",
                  )}
                >
                  <div className="flex items-center flex-1">
                    <div className="w-12 h-12 bg-bg-color rounded-full flex items-center justify-center mr-4">
                      {d.type === "laptop" ? (
                        <Laptop className="w-6 h-6 text-text-main" />
                      ) : (
                        <Tablet className="w-6 h-6 text-text-main" />
                      )}
                    </div>
                    <div className="flex-1">
                      <h4 className="text-[16px] font-medium text-text-main">
                        {d.name}
                      </h4>
                      <p className="text-[13px] text-text-sub mt-0.5">
                        {d.lastActive}
                      </p>
                    </div>
                  </div>
                  <button
                    className="text-[14px] text-[#FA5151] font-medium active:opacity-70 px-3 py-1.5 rounded-full bg-[#FA5151]/10"
                    onClick={() => handleLogout(d.id)}
                  >{t('user.auto_9f214', '下线')}</button>
                </div>
              ))}
            </div>
          </>
        )}
      </div>
    </PageLayout>
  );
};

export const SwitchAccount = () => {
  const { t } = useTranslation();
  
return (
    <PageLayout title={t('user.auto_prop_26d01b0c', '切换账号')}>
      <div className="flex flex-col items-center py-10">
        <Avatar
          src="https://picsum.photos/seed/me/200/200"
          size="lg"
          className="w-20 h-20 rounded-full mb-4"
        />
        <h3 className="text-[18px] font-medium text-text-main mb-8">{t('user.auto_2c9b5a6b', '当前账号')}</h3>
        <button
          className="w-[200px] h-12 bg-chat-other-bg text-text-main rounded-lg font-medium active:bg-active-bg transition-colors mb-4 border border-border-color"
          onClick={async () => {
            const acc = await showPrompt("请输入要添加的微信号/手机号");
            if (acc) showToast(t('user.auto_fn_292d1306', '添加成功并自动切换！'));
          }}
        >{t('user.auto_7e69752b', '+ 添加账号')}</button>
      </div>
    </PageLayout>
  );
};
