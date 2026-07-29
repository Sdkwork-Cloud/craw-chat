import type { ComponentType } from "react";
import { CircleUserRound } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

function createUnavailableUserPage(displayName: string): ComponentType<Record<string, unknown>> {
  function UnavailableUserPage() {
    const navigate = useNavigate();
    const { t } = useTranslation("user");

    return (
      <CapabilityUnavailablePage
        icon={CircleUserRound}
        message={t("capability_unavailable")}
        onBack={() => navigate(-1)}
        title={t("capability_title")}
      />
    );
  }

  UnavailableUserPage.displayName = displayName;
  return UnavailableUserPage;
}

export const BillingRecordsPage = createUnavailableUserPage("BillingRecordsPage");
export const CreateCharacter = createUnavailableUserPage("CreateCharacter");
export const CreateVoice = createUnavailableUserPage("CreateVoice");
export const Discover = createUnavailableUserPage("Discover");
export const EmojiPage = createUnavailableUserPage("EmojiPage");
export const FavoritesPage = createUnavailableUserPage("FavoritesPage");
export const Me = createUnavailableUserPage("Me");
export const MyAgentsPage = createUnavailableUserPage("MyAgentsPage");
export const MyCharacterDetail = createUnavailableUserPage("MyCharacterDetail");
export const MyCharacters = createUnavailableUserPage("MyCharacters");
export const MyProfile = createUnavailableUserPage("MyProfile");
export const MyVoiceDetail = createUnavailableUserPage("MyVoiceDetail");
export const MyVoices = createUnavailableUserPage("MyVoices");
export const MyWorksPage = createUnavailableUserPage("MyWorksPage");
export const ServicesPage = createUnavailableUserPage("ServicesPage");
export const WorkDetailPage = createUnavailableUserPage("WorkDetailPage");
export const WorkEditPage = createUnavailableUserPage("WorkEditPage");
export const Workspace = createUnavailableUserPage("Workspace");
export const GamesPage = createUnavailableUserPage("GamesPage");
export const MomentsPage = createUnavailableUserPage("MomentsPage");
export const SearchPage = createUnavailableUserPage("SearchPage");
export const ProfileAddress = createUnavailableUserPage("ProfileAddress");
export const ProfileAvatar = createUnavailableUserPage("ProfileAvatar");
export const ProfileBeans = createUnavailableUserPage("ProfileBeans");
export const ProfileMore = createUnavailableUserPage("ProfileMore");
export const ProfileName = createUnavailableUserPage("ProfileName");
export const ProfileQRCode = createUnavailableUserPage("ProfileQRCode");
export const ProfileRingtone = createUnavailableUserPage("ProfileRingtone");
export const ProfileTickle = createUnavailableUserPage("ProfileTickle");
export const About = createUnavailableUserPage("About");
export const Blacklist = createUnavailableUserPage("Blacklist");
export const ChatSettings = createUnavailableUserPage("ChatSettings");
export const Complain = createUnavailableUserPage("Complain");
export const ElderlyMode = createUnavailableUserPage("ElderlyMode");
export const FAQ = createUnavailableUserPage("FAQ");
export const Features = createUnavailableUserPage("Features");
export const Feedback = createUnavailableUserPage("Feedback");
export const FriendPermissions = createUnavailableUserPage("FriendPermissions");
export const Gender = createUnavailableUserPage("Gender");
export const General = createUnavailableUserPage("General");
export const HelpFeedback = createUnavailableUserPage("HelpFeedback");
export const InfoCollection = createUnavailableUserPage("InfoCollection");
export const ManageChatHistory = createUnavailableUserPage("ManageChatHistory");
export const Notifications = createUnavailableUserPage("Notifications");
export const Plugins = createUnavailableUserPage("Plugins");
export const Privacy = createUnavailableUserPage("Privacy");
export const PrivacyPolicy = createUnavailableUserPage("PrivacyPolicy");
export const Region = createUnavailableUserPage("Region");
export const Signature = createUnavailableUserPage("Signature");
export const TeenMode = createUnavailableUserPage("TeenMode");
export const ThirdPartySharing = createUnavailableUserPage("ThirdPartySharing");
export const TOS = createUnavailableUserPage("TOS");
export const AdManagement = createUnavailableUserPage("AdManagement");
export const ChatBackground = createUnavailableUserPage("ChatBackground");
export const ClearChatHistory = createUnavailableUserPage("ClearChatHistory");
export const EmojiManagement = createUnavailableUserPage("EmojiManagement");
export const FontSize = createUnavailableUserPage("FontSize");
export const MediaSettings = createUnavailableUserPage("MediaSettings");
export const StorageSpace = createUnavailableUserPage("StorageSpace");
export const SystemPermissions = createUnavailableUserPage("SystemPermissions");
