import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';
import React, { useState } from "react";
import {} from "react-router";
import { PageLayout, Group, ListItem, ToggleItem } from "./SettingsSubPages";
import { showToast, showPrompt } from "@sdkwork/im-h5-commons";

export const ChangePhoneNumber = () => {
  const { t } = useTranslation();
const [step, setStep] = useState(1);
  const [phone, setPhone] = useState("");
  const [code, setCode] = useState("");

  const handleSubmit = () => {
  if (!phone || !code) return showToast(t("user:account_sec.enter_full_info", "请输入完整信息"));
    showToast(t("user:account_sec.phone_changed", "手机号已更变"));
    // normally navigate back
  };

  return (
    <PageLayout title={t("user:account_sec.bind_phone", "绑定手机号")}>
      {step === 1 ? (
        <div className="flex flex-col items-center py-10 px-4">
          <div className="w-16 h-16 bg-primary-blue/10 rounded-full flex items-center justify-center mb-6">
            <span className="text-primary-blue text-3xl">📱</span>
          </div>
          <h3 className="text-[18px] font-medium text-text-main mb-2">{t("user:account_sec.your_phone", "你的手机号码：+86 138****8888")}</h3>
          <p className="text-[14px] text-text-sub text-center mb-8">{t("user:account_sec.bind_phone_desc", "绑定的手机号可用于登录 Sdkwork IM H5，或找回密码。")}</p>
          <button
            onClick={() => setStep(2)}
            className="w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          >{t("user:account_sec.change_phone", "更换手机号")}</button>
        </div>
      ) : (
        <div className="px-4 py-6">
          <h3 className="text-[20px] font-medium text-text-main mb-6">{t("user:account_sec.verify_new_phone", "验证新手机号")}</h3>
          <div className="flex items-center border-b border-border-color py-3 mb-4">
            <span className="text-[16px] text-text-main mr-4">+86</span>
            <input
              type="tel"
              placeholder={t("user:account_sec.enter_phone", "请填写手机号")}
              value={phone}
              onChange={(e) => setPhone(e.target.value)}
              className="flex-1 bg-transparent text-[16px] text-text-main outline-none"
            />
          </div>
          <div className="flex items-center border-b border-border-color py-3 mb-8">
            <input
              type="text"
              placeholder={t("user:account_sec.verification_code", "验证码")}
              value={code}
              onChange={(e) => setCode(e.target.value)}
              className="flex-1 bg-transparent text-[16px] text-text-main outline-none"
            />
            <button
              className="text-[#00B42A] text-[15px] font-medium ml-4"
              onClick={() => showToast(t("user:account_sec.code_sent", "验证码已发送"))}
            >{t("user:account_sec.get_code", "获取验证码")}</button>
          </div>
          <button
            className="w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
            onClick={handleSubmit}
          >{t("user:account_sec.submit", "提交")}</button>
        </div>
      )}
    </PageLayout>
  );
};

export const ChangePassword = () => {
  const { t } = useTranslation();
  
return (
    <PageLayout title={t("user:account_sec.set_password", "设置密码")}>
      <div className="px-4 py-6">
        <div className="border-b border-border-color py-3 mb-2">
          <input
            type="password"
            placeholder={t("user:account_sec.enter_old_pwd", "请填写原密码")}
            className="w-full bg-transparent text-[16px] text-text-main outline-none"
          />
        </div>
        <div className="border-b border-border-color py-3 mb-2">
          <input
            type="password"
            placeholder={t("user:account_sec.enter_new_pwd", "请填写新密码")}
            className="w-full bg-transparent text-[16px] text-text-main outline-none"
          />
        </div>
        <div className="border-b border-border-color py-3 mb-8">
          <input
            type="password"
            placeholder={t("user:account_sec.confirm_new_pwd", "请再次填写新密码")}
            className="w-full bg-transparent text-[16px] text-text-main outline-none"
          />
        </div>
        <p className="text-[13px] text-text-sub mb-8">{t("user:account_sec.pwd_requirements", "密码必须包含字母和数字，且长度不少于8位。")}</p>
        <button
          className="w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={() => showToast(t("user:account_sec.operation_executed", "操作已执行"))}
        >{t("user:account_sec.done", "完成")}</button>
      </div>
    </PageLayout>
  );
};

export const VoiceLock = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  const [enabled, setEnabled] = useState(false);
  return (
    <PageLayout title={t("user:account_sec.voice_lock", "声音锁")}>
      <Group className="mt-4">
        <ToggleItem
          label={t("user:account_sec.login_sdkwork_im_h5", "登录 Sdkwork IM H5")}
          checked={enabled}
          onChange={setEnabled}
          hideBorder
        />
      </Group>
      <p className="text-[13px] text-text-sub px-4 mb-8">{t("user:account_sec.voice_lock_desc", "开启后，可以使用声音解锁应用或验证身份。")}</p>
      <Group>
        <ListItem
          label={t("user:account_sec.reset_voice_lock", "重设声音锁")}
          hideBorder
          onClick={() => navigate("/settings/account/voice-lock/reset")}
        />
      </Group>
    </PageLayout>
  );
};

export const EmergencyContacts = () => {
  const { t } = useTranslation();
  
const [contacts, setContacts] = useState([
    { name: t("user:account_sec.dad", "爸爸"), phone: "138****0001", relation: t("user:account_sec.father", "父亲") },
    { name: t("user:account_sec.friend_name", "李小明"), phone: "139****0002", relation: t("user:account_sec.friend", "朋友") },
  ]);

  return (
    <PageLayout title={t("user:account_sec.emergency_contacts", "应急联系人")}>
      <div className="flex flex-col h-full bg-bg-color">
        <div className="p-4 bg-chat-other-bg border-b border-border-color">
          <p className="text-[14px] text-text-sub leading-relaxed">{t("user:account_sec.emergency_desc_long", "当你的账号存在安全风险或无法登录时，可通过应急联系人辅助验证身份，恢复账号访问权限。")}</p>
        </div>

        <div className="flex-1 overflow-y-auto w-full mt-2">
          {contacts.map((contact, i) => (
            <div
              key={i}
              className="flex justify-between items-center p-4 bg-chat-other-bg border-b border-border-color active:bg-active-bg transition-colors"
            >
              <div>
                <span className="text-[16px] font-medium text-text-main flex items-center gap-2">
                  {contact.name}
                  <span className="text-[11px] bg-primary-blue/10 text-primary-blue px-1.5 py-0.5 rounded-sm">
                    {contact.relation}
                  </span>
                </span>
                <p className="text-[13px] text-text-sub mt-1">
                  {contact.phone}
                </p>
              </div>
              <button
                className="text-[13px] text-[#FA5151] px-3 py-1.5 rounded-full border border-border-color active:opacity-70"
                onClick={async () => {
                  showToast(t("user:account_sec.contact_removed", "已移除联系人"));
                  setContacts(contacts.filter((_, idx) => idx !== i));
                }}
              >{t("user:account_sec.remove", "移除")}</button>
            </div>
          ))}

          <div className="p-6 flex justify-center">
            <button
              className="w-full h-12 bg-chat-other-bg text-text-main rounded-xl font-medium active:bg-active-bg transition-colors border border-border-color flex justify-center items-center gap-2"
              onClick={async () => {
                const name = await showPrompt(t("user:account_sec.enter_contact_name", "请输入应急联系人姓名"));
                if (name) {
                  const phone = await showPrompt(t("user:account_sec.enter_contact_phone", "请输入联系人手机号"));
                  if (phone) {
                    setContacts([
                      ...contacts,
                      { name, phone, relation: t("user:account_sec.friend", "朋友") },
                    ]);
                    showToast(t("user:account_sec.add_success", "添加成功"));
                  }
                }
              }}
            >{t("user:account_sec.add_contact_btn", "添加应急联系人")}</button>
          </div>
        </div>
      </div>
    </PageLayout>
  );
};

export const MoreSecurity = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  return (
    <PageLayout title={t("user:account_sec.more_sec_settings", "更多安全设置")}>
      <Group>
        <ListItem
          label={t("user:account_sec.qq", "QQ号")}
          rightText={t("user:account_sec.not_bound", "未绑定")}
          onClick={() => navigate("/settings/account/more/qq")}
        />
        <ListItem
          label={t("user:account_sec.email", "邮件地址")}
          rightText={t("user:account_sec.not_bound", "未绑定")}
          hideBorder
          onClick={() => navigate("/settings/account/more/email")}
        />
      </Group>
      <Group>
        <ListItem
          label={t("user:account_sec.recover_pwd", "恢复账号密码")}
          hideBorder
          onClick={() => navigate("/settings/account/more/recover")}
        />
      </Group>
      <Group>
        <ListItem
          label={t("user:account_sec.delete_account", "注销账号")}
          hideBorder
          onClick={() => navigate("/settings/account/more/delete")}
        />
      </Group>
    </PageLayout>
  );
};
