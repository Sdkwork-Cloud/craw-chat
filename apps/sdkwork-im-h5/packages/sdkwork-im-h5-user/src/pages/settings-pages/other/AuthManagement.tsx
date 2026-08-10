import React, { useState } from "react";
import { useTranslation } from 'react-i18next';
import { PageLayout } from "../../SettingsSubPages";
import { showToast } from "@sdkwork/im-h5-commons";

export const AuthManagement: React.FC = () => {
  const { t } = useTranslation();
  const [apps, setApps] = useState([
    {
      id: 1,
      name: "WPS 办公助手",
      desc: "获取你的基础信息(昵称、头像)",
      color: "bg-blue-500",
      letter: "W",
    },
    {
      id: 2,
      name: "滴滴出行",
      desc: "获取你的位置信息和基础信息",
      color: "bg-orange-500",
      letter: "D",
    },
    {
      id: 3,
      name: "京东购物",
      desc: "获取你的基础信息",
      color: "bg-red-500",
      letter: "J",
    },
  ]);

  return (
    <PageLayout title={t('user.auto_prop_2ed19e80', "授权管理")}>
      <div className="p-4">
        <h3 className="text-[13px] text-text-sub mb-2 ml-1">
          {t('user.auto_3d803059', `你已授权以下应用`)}
        </h3>
        <div className="bg-chat-other-bg rounded-xl overflow-hidden">
          {apps.map((app) => (
            <div
              key={app.id}
              className="flex items-center justify-between p-4 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer border-b border-border-color last:border-0"
            >
              <div className="flex items-center">
                <div
                  className={`w-12 h-12 ${app.color} rounded-lg flex items-center justify-center mr-4`}
                >
                  <span className="text-white font-bold">{app.letter}</span>
                </div>
                <div>
                  <h4 className="text-[16px] font-medium text-text-main">
                    {app.name}
                  </h4>
                  <p className="text-[13px] text-text-sub mt-0.5">{app.desc}</p>
                </div>
              </div>
              <button
                className="text-[14px] text-accent-red font-medium active:opacity-70 px-3 py-1.5 rounded-full bg-accent-red/10"
                onClick={() => {
                  setApps(apps.filter((x) => x.id !== app.id));
                  showToast("已解除授权");
                }}
              >
                解除
              </button>
            </div>
          ))}
          {apps.length === 0 && (
            <div className="p-8 text-center text-text-sub">
              {t('user.auto_49721453', `暂无授权应用`)}
            </div>
          )}
        </div>
        <p className="text-[13px] text-text-sub text-center mt-6">
          {t('user.auto_60aa553e', `以上应用可通过 ClawChat 快速登录并获取相关信息。`)}
        </p>
      </div>
    </PageLayout>
  );
};
