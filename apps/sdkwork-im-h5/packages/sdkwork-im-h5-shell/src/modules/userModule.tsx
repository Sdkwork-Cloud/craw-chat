import React from "react";
import { LayoutGrid, Compass, UserRound } from "lucide-react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";
import { TabSolidWorkspace, TabSolidDiscover, TabSolidUser } from "../navigation/solidTabIcons";

type ComponentName = "Workspace" | "Discover" | "Me" | "MyProfile" | "ProfileAvatar" | "ProfileName" | "ProfileTickle" | "ProfileQRCode" | "ProfileMore" | "Gender" | "Region" | "Signature" | "ProfileRingtone" | "ProfileBeans" | "ProfileAddress" | "SettingsPage" | "AccountSecurity" | "WechatID" | "ChangePhoneNumber" | "ChangePassword" | "VoiceLock" | "ResetVoiceLock" | "EmergencyContacts" | "MoreSecurity" | "BindQQ" | "BindEmail" | "RecoverPassword" | "DeleteAccount" | "TeenMode" | "ElderlyMode" | "Notifications" | "ChatSettings" | "ChatBackground" | "EmojiManagement" | "ClearChatHistory" | "Devices" | "General" | "FontSize" | "MediaSettings" | "StorageSpace" | "ManageChatHistory" | "FriendPermissions" | "Blacklist" | "Privacy" | "SystemPermissions" | "AuthManagement" | "AdManagement" | "InfoCollection" | "ThirdPartySharing" | "Plugins" | "HelpFeedback" | "FAQ" | "Feedback" | "About" | "Features" | "Complain" | "TOS" | "PrivacyPolicy" | "SwitchAccount" | "ServicesPage" | "BillingRecordsPage" | "FavoritesPage" | "MyWorksPage" | "WorkDetailPage" | "WorkEditPage" | "EmojiPage" | "ChannelsPage" | "SearchPage" | "GamesPage";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-user");
    return { default: mod[name] };
  });
}

const Workspace = lazyComponent("Workspace");
const Discover = lazyComponent("Discover");
const Me = lazyComponent("Me");
const MyProfile = lazyComponent("MyProfile");
const ProfileAvatar = lazyComponent("ProfileAvatar");
const ProfileName = lazyComponent("ProfileName");
const ProfileTickle = lazyComponent("ProfileTickle");
const ProfileQRCode = lazyComponent("ProfileQRCode");
const ProfileMore = lazyComponent("ProfileMore");
const Gender = lazyComponent("Gender");
const Region = lazyComponent("Region");
const Signature = lazyComponent("Signature");
const ProfileRingtone = lazyComponent("ProfileRingtone");
const ProfileBeans = lazyComponent("ProfileBeans");
const ProfileAddress = lazyComponent("ProfileAddress");
const SettingsPage = lazyComponent("SettingsPage");
const AccountSecurity = lazyComponent("AccountSecurity");
const WechatID = lazyComponent("WechatID");
const ChangePhoneNumber = lazyComponent("ChangePhoneNumber");
const ChangePassword = lazyComponent("ChangePassword");
const VoiceLock = lazyComponent("VoiceLock");
const ResetVoiceLock = lazyComponent("ResetVoiceLock");
const EmergencyContacts = lazyComponent("EmergencyContacts");
const MoreSecurity = lazyComponent("MoreSecurity");
const BindQQ = lazyComponent("BindQQ");
const BindEmail = lazyComponent("BindEmail");
const RecoverPassword = lazyComponent("RecoverPassword");
const DeleteAccount = lazyComponent("DeleteAccount");
const TeenMode = lazyComponent("TeenMode");
const ElderlyMode = lazyComponent("ElderlyMode");
const Notifications = lazyComponent("Notifications");
const ChatSettings = lazyComponent("ChatSettings");
const ChatBackground = lazyComponent("ChatBackground");
const EmojiManagement = lazyComponent("EmojiManagement");
const ClearChatHistory = lazyComponent("ClearChatHistory");
const Devices = lazyComponent("Devices");
const General = lazyComponent("General");
const FontSize = lazyComponent("FontSize");
const MediaSettings = lazyComponent("MediaSettings");
const StorageSpace = lazyComponent("StorageSpace");
const ManageChatHistory = lazyComponent("ManageChatHistory");
const FriendPermissions = lazyComponent("FriendPermissions");
const Blacklist = lazyComponent("Blacklist");
const Privacy = lazyComponent("Privacy");
const SystemPermissions = lazyComponent("SystemPermissions");
const AuthManagement = lazyComponent("AuthManagement");
const AdManagement = lazyComponent("AdManagement");
const InfoCollection = lazyComponent("InfoCollection");
const ThirdPartySharing = lazyComponent("ThirdPartySharing");
const Plugins = lazyComponent("Plugins");
const HelpFeedback = lazyComponent("HelpFeedback");
const FAQ = lazyComponent("FAQ");
const Feedback = lazyComponent("Feedback");
const About = lazyComponent("About");
const Features = lazyComponent("Features");
const Complain = lazyComponent("Complain");
const TOS = lazyComponent("TOS");
const PrivacyPolicy = lazyComponent("PrivacyPolicy");
const SwitchAccount = lazyComponent("SwitchAccount");
// My voice library pages are owned by sdkwork-voice and bridged through the
// IM H5 AI Voice adapter (canonical voice mobile UI lives in sdkwork-voice).
const MyVoices = React.lazy(async () => {
  const mod = await import("@sdkwork/im-h5-ai-voice");
  return { default: mod.MyVoicesPage };
});
const CreateVoice = React.lazy(async () => {
  const mod = await import("@sdkwork/im-h5-ai-voice");
  return { default: mod.CreateVoicePage };
});
const MyVoiceDetail = React.lazy(async () => {
  const mod = await import("@sdkwork/im-h5-ai-voice");
  return { default: mod.MyVoiceDetailPage };
});
const ServicesPage = lazyComponent("ServicesPage");
const BillingRecordsPage = lazyComponent("BillingRecordsPage");
const FavoritesPage = lazyComponent("FavoritesPage");
const MyWorksPage = lazyComponent("MyWorksPage");
const WorkDetailPage = lazyComponent("WorkDetailPage");
const WorkEditPage = lazyComponent("WorkEditPage");
const EmojiPage = lazyComponent("EmojiPage");
const ChannelsPage = lazyComponent("ChannelsPage");
const SearchPage = lazyComponent("SearchPage");
const GamesPage = lazyComponent("GamesPage");

export const userModule: ImH5CapabilityModule = {
  id: "user",
  navigation: [
    { id: "workspace", moduleId: "user", path: "/workspace", labelKey: "common.tabs.workspace", icon: LayoutGrid, activeIcon: TabSolidWorkspace },
    { id: "discover", moduleId: "user", path: "/discover", labelKey: "common.tabs.discover", icon: Compass, activeIcon: TabSolidDiscover },
    { id: "me", moduleId: "user", path: "/me", labelKey: "common.tabs.me", icon: UserRound, activeIcon: TabSolidUser },
  ],
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.userWorkspace, render: () => <Workspace /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userDiscover, render: () => <Discover /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userMe, render: () => <Me /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userProfile, render: () => <MyProfile /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userProfileAvatar, render: () => <ProfileAvatar /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userProfileName, render: () => <ProfileName /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userProfileTickle, render: () => <ProfileTickle /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userProfileQrCode, render: () => <ProfileQRCode /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userProfileMore, render: () => <ProfileMore /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userProfileGender, render: () => <Gender /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userProfileRegion, render: () => <Region /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userProfileSignature, render: () => <Signature /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userProfileRingtone, render: () => <ProfileRingtone /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userProfileBeans, render: () => <ProfileBeans /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userProfileAddress, render: () => <ProfileAddress /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettings, render: () => <SettingsPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsAccount, render: () => <AccountSecurity /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsWechatId, render: () => <WechatID /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsPhone, render: () => <ChangePhoneNumber /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsPassword, render: () => <ChangePassword /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsVoiceLock, render: () => <VoiceLock /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsVoiceLockReset, render: () => <ResetVoiceLock /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsEmergency, render: () => <EmergencyContacts /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsMoreSecurity, render: () => <MoreSecurity /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsBindQq, render: () => <BindQQ /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsBindEmail, render: () => <BindEmail /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsRecoverPassword, render: () => <RecoverPassword /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsDeleteAccount, render: () => <DeleteAccount /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsTeenMode, render: () => <TeenMode /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsElderlyMode, render: () => <ElderlyMode /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsNotifications, render: () => <Notifications /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsChat, render: () => <ChatSettings /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsChatBackground, render: () => <ChatBackground /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsChatEmoji, render: () => <EmojiManagement /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsChatClear, render: () => <ClearChatHistory /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsDevices, render: () => <Devices /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsGeneral, render: () => <General /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsFontSize, render: () => <FontSize /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsMedia, render: () => <MediaSettings /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsStorage, render: () => <StorageSpace /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsManageChatHistory, render: () => <ManageChatHistory /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsFriendPermissions, render: () => <FriendPermissions /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsBlacklist, render: () => <Blacklist /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsPrivacy, render: () => <Privacy /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsPrivacySystem, render: () => <SystemPermissions /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsPrivacyAuth, render: () => <AuthManagement /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsPrivacyAds, render: () => <AdManagement /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsInfoCollection, render: () => <InfoCollection /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsThirdPartySharing, render: () => <ThirdPartySharing /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsPlugins, render: () => <Plugins /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsHelp, render: () => <HelpFeedback /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsHelpFaq, render: () => <FAQ /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsHelpFeedback, render: () => <Feedback /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsAbout, render: () => <About /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsAboutFeatures, render: () => <Features /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsAboutComplain, render: () => <Complain /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsAboutTos, render: () => <TOS /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsAboutPrivacy, render: () => <PrivacyPolicy /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userSettingsSwitchAccount, render: () => <SwitchAccount /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userVoices, render: () => <MyVoices /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userVoicesCreate, render: () => <CreateVoice /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userVoicesDetail, render: () => <MyVoiceDetail /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userServices, render: () => <ServicesPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userBillingRecords, render: () => <BillingRecordsPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userFavorites, render: () => <FavoritesPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userMyWorks, render: () => <MyWorksPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userWorkDetail, render: () => <WorkDetailPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userWorkEdit, render: () => <WorkEditPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userEmoji, render: () => <EmojiPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userDiscoverChannels, render: () => <ChannelsPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userDiscoverSearch, render: () => <SearchPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.userDiscoverGames, render: () => <GamesPage /> },
  ],
};
