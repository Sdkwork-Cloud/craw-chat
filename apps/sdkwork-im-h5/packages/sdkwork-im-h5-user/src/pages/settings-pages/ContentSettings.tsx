import { useTranslation } from 'react-i18next';
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { SettingsService } from "../../services/SettingsService";
import {
  PageLayout,
  Group,
  ListItem,
  ToggleItem,
} from "../../components/SettingsCommons";

export const Notifications = () => {
  const { t } = useTranslation();
const [sound, setSound] = useState(true);
  const [vibrate, setVibrate] = useState(true);
  const [preview, setPreview] = useState(true);
  const [newMsg, setNewMsg] = useState(true);
  const [callInvite, setCallInvite] = useState(true);

  return (
    <PageLayout title={t('user.auto_prop_n33c3c11e', 'New message notifications')}>
      <Group>
        <ToggleItem
          label={t('user.auto_prop_n5cd900af', 'Receive new message notifications')}
          checked={newMsg}
          onChange={setNewMsg}
        />
        <ToggleItem
          label={t('user.auto_prop_3d755865', 'Receive voice and video call invitation notifications')}
          checked={callInvite}
          onChange={setCallInvite}
          hideBorder
        />
      </Group>
      <Group>
        <ToggleItem
          label={t('user.auto_prop_n1ab714b3', 'Show message details in notifications')}
          checked={preview}
          onChange={setPreview}
          hideBorder
        />
      </Group>
      <Group>
        <ToggleItem label={t('user.auto_prop_b5d03', 'Sounds')} checked={sound} onChange={setSound} />
        <ToggleItem
          label={t('user.auto_prop_129c81', 'Vibration')}
          checked={vibrate}
          onChange={setVibrate}
          hideBorder
        />
      </Group>
    </PageLayout>
  );
};

export const ChatSettings = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  return (
    <PageLayout title={t('user.auto_prop_fe21f', 'Chats')}>
      <Group>
        <ListItem
          label={t('user.auto_prop_3bafd582', 'Chat wallpaper')}
          hideBorder
          onClick={() => navigate("/settings/chat/background")}
        />
      </Group>
      <Group>
        <ListItem
          label={t('user.auto_prop_3f7c6ea2', 'Sticker management')}
          hideBorder
          onClick={() => navigate("/settings/chat/emoji")}
        />
      </Group>
      <Group>
        <ListItem
          label={t('user.auto_prop_n48c7a567', 'Clear chat history')}
          hideBorder
          onClick={() => navigate("/settings/chat/clear")}
        />
      </Group>
    </PageLayout>
  );
};

export const General = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  const [landscape, setLandscape] = useState(false);
  const [isDark, setIsDark] = useState(false);

  useEffect(() => {
    SettingsService.getSettings().then((s) => setLandscape(s.landscape));
    const isDarkMode = document.documentElement.classList.contains("dark");
    setIsDark(isDarkMode);
  }, []);

  const handleLandscapeToggle = async (val: boolean) => {
    setLandscape(val);
    await SettingsService.updateSettings({ landscape: val });
  };

  const handleThemeToggle = (checked: boolean) => {
    setIsDark(checked);
    document.documentElement.classList.toggle("dark", checked);
    void SettingsService.updateSettings({ darkMode: checked });
  };

  return (
    <PageLayout title={t('user.auto_prop_11e84e', 'General')}>
      <Group>
        <ToggleItem
          label={t('user.auto_prop_33f0e76f', 'Dark mode')}
          checked={isDark}
          onChange={handleThemeToggle}
        />
        <ListItem label={t('user.auto_prop_15ff64d', 'Language')} rightText={t('user.auto_prop_3957780d', 'Simplified Chinese')} />
        <ListItem
          label={t('user.auto_prop_2aba3fa4', 'Font size')}
          hideBorder
          onClick={() => navigate("/settings/general/font-size")}
        />
      </Group>
      <Group>
        <ListItem
          label={t('user.auto_prop_n1e44caf5', 'Photos, videos, files and calls')}
          hideBorder
          onClick={() => navigate("/settings/general/media")}
        />
      </Group>
      <Group>
        <ToggleItem
          label={t('user.auto_prop_nfa62f1e', 'Landscape mode')}
          checked={landscape}
          onChange={handleLandscapeToggle}
          hideBorder
        />
      </Group>
      <Group>
        <ListItem
          label={t('user.auto_prop_2ac3f70a', 'Storage')}
          hideBorder
          onClick={() => navigate("/settings/general/storage")}
        />
      </Group>
    </PageLayout>
  );
};

export const Plugins = () => {
  const { t } = useTranslation();
  
const [kanYiKan, setKanYiKan] = useState(true);
  const [souYiSou, setSouYiSou] = useState(true);
  return (
    <PageLayout title={t("settings:content.plugins", "Plugins")}>
      <Group>
        <div className="flex items-center px-4 py-3.5 bg-chat-other-bg border-b border-border-color/60">
          <div className="w-10 h-10 bg-green-500 rounded-lg flex items-center justify-center mr-3">
            <span className="text-white text-xl">📰</span>
          </div>
          <div className="flex-1">
            <h4 className="text-[16px] text-text-main">{t("settings:content.top_stories", "Top Stories")}</h4>
            <p className="text-[13px] text-text-sub">{t("settings:content.top_stories_desc", "Discover what your friends are following")}</p>
          </div>
          <div
            className={`w-12 h-6 rounded-full relative cursor-pointer transition-colors ${kanYiKan ? "bg-accent-green" : "bg-gray-300 dark:bg-gray-600"}`}
            onClick={() => setKanYiKan(!kanYiKan)}
          >
            <div
              className={`absolute top-1 w-4 h-4 rounded-full bg-white transition-transform ${kanYiKan ? "left-7" : "left-1"}`}
            />
          </div>
        </div>
        <div className="flex items-center px-4 py-3.5 bg-chat-other-bg">
          <div className="w-10 h-10 bg-orange-500 rounded-lg flex items-center justify-center mr-3">
            <span className="text-white text-xl">🔍</span>
          </div>
          <div className="flex-1">
            <h4 className="text-[16px] text-text-main">{t("settings:content.search_all", "Search")}</h4>
            <p className="text-[13px] text-text-sub">{t("settings:content.search_all_desc", "Search articles, mini programs, etc.")}</p>
          </div>
          <div
            className={`w-12 h-6 rounded-full relative cursor-pointer transition-colors ${souYiSou ? "bg-accent-green" : "bg-gray-300 dark:bg-gray-600"}`}
            onClick={() => setSouYiSou(!souYiSou)}
          >
            <div
              className={`absolute top-1 w-4 h-4 rounded-full bg-white transition-transform ${souYiSou ? "left-7" : "left-1"}`}
            />
          </div>
        </div>
      </Group>
    </PageLayout>
  );
};
