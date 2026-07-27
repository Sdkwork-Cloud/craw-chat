import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { showToast, showPrompt } from "@sdkwork/im-h5-commons";
import {
  ProfileService,
  type UserProfile,
} from "../../services/ProfileService";
import { PageLayout } from "../../components/SettingsCommons";

export const ProfileBeans = () => {
  const { t } = useTranslation();
const [profile, setProfile] = useState<UserProfile | null>(null);

  useEffect(() => {
    ProfileService.getUserProfile().then(setProfile);
  }, []);

  return (
    <PageLayout title={t('user.auto_prop_17164b3', '微信豆')}>
      <div className="flex flex-col items-center py-10">
        <div className="w-20 h-20 bg-yellow-500/10 rounded-full flex items-center justify-center mb-4">
          <span className="text-yellow-500 text-3xl">💰</span>
        </div>
        <h3 className="text-[32px] font-bold text-text-main mb-1">
          {profile?.beans || 0}
        </h3>
        <p className="text-[14px] text-text-sub mb-8">{t('user.auto_2c94233e', '当前余额')}</p>
        <button
          className="w-[200px] h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={async () => {
            const amount = await showPrompt("请输入充值数量", "100");
            if (amount && !isNaN(Number(amount)) && Number(amount) > 0) {
              const pass = await showPrompt(
                "请输入6位支付密码（模拟）",
                "123456",
              );
              if (pass && pass.length === 6) {
                showToast(t('user.auto_fn_n60cfc4a5', '充值成功！'));
                setProfile((p) =>
                  p ? { ...p, beans: (p.beans || 0) + Number(amount) } : p,
                );
              } else {
                showToast(t('user.auto_fn_n2e3dc594', '支付取消或密码错误'));
              }
            }
          }}
        >{t('user.auto_a2797', '充值')}</button>
      </div>
    </PageLayout>
  );
};

export const ProfileAddress = () => {
  const { t } = useTranslation();
  
const [addresses, setAddresses] = useState([
    {
      id: 1,
      name: "张三",
      phone: "138****0001",
      address: "北京市朝阳区建国路88号 Soho现代城 3号楼2单元1204室",
      isDefault: true,
    },
    {
      id: 2,
      name: "李老师",
      phone: "139****0002",
      address: "上海市浦东新区陆家嘴环路1000号 环球金融中心 88层",
      isDefault: false,
    },
  ]);

  return (
    <PageLayout title={t('user.auto_prop_2e5be3e3', '我的地址')}>
      <div className="flex flex-col h-full bg-bg-color w-full">
        <div className="flex-1 overflow-y-auto w-full pt-2">
          {addresses.map((addr) => (
            <div
              key={addr.id}
              className="flex justify-between items-start p-4 bg-chat-other-bg border-b border-border-color active:bg-active-bg transition-colors"
            >
              <div className="flex-1 mr-4">
                <div className="flex items-center gap-2 mb-1.5">
                  <h3 className="text-[16px] font-bold text-text-main">
                    {addr.name}
                  </h3>
                  <span className="text-[15px] font-medium text-text-sub">
                    {addr.phone}
                  </span>
                  {addr.isDefault && (
                    <span className="text-[10px] bg-primary-blue/10 text-primary-blue px-1.5 py-0.5 rounded-sm shrink-0 border border-primary-blue/30">{t('user.auto_13c7cc', '默认')}</span>
                  )}
                </div>
                <p className="text-[14px] text-text-main leading-relaxed">
                  {addr.address}
                </p>
              </div>
              <div className="w-[1px] h-10 bg-border-color shrink-0 self-center mr-4" />
              <button
                className="text-[14px] text-text-sub shrink-0 self-center font-medium active:opacity-70 h-10 flex items-center justify-center"
                onClick={async () => {
                  const newAddr = await showPrompt(
                    "修改地址详情",
                    addr.address,
                  );
                  if (newAddr) {
                    setAddresses(
                      addresses.map((a) =>
                        a.id === addr.id ? { ...a, address: newAddr } : a,
                      ),
                    );
                    showToast(t('user.auto_fn_25ddaeda', '修改成功'));
                  }
                }}
              >{t('user.auto_ff33b', '编辑')}</button>
            </div>
          ))}

          <div className="p-6 flex justify-center mt-4">
            <button
              className="w-full h-12 bg-primary-blue text-white rounded-xl font-medium active:opacity-80 transition-opacity flex justify-center items-center gap-2"
              onClick={async () => {
                const name = await showPrompt("请输入联系人姓名");
                if (name) {
                  const phone = await showPrompt("请输入联系人手机");
                  if (phone) {
                    const address = await showPrompt("请输入详细地址");
                    if (address) {
                      setAddresses([
                        ...addresses,
                        {
                          id: Date.now(),
                          name,
                          phone,
                          address,
                          isDefault: false,
                        },
                      ]);
                      showToast(t('user.auto_fn_3340e954', '添加成功'));
                    }
                  }
                }
              }}
            >{t('user.auto_7ab45e53', '+ 新增地址')}</button>
          </div>
        </div>
      </div>
    </PageLayout>
  );
};
