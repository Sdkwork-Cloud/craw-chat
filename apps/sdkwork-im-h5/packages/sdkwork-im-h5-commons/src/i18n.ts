import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

import zhCommon from './locales/zh/common.json';
import zhAgents from './locales/zh/agents.json';
import zhAuth from './locales/zh/auth.json';
import zhShopping from './locales/zh/shopping.json';
import zhCommons from './locales/zh/commons.json';
import zhWorkspace from './locales/zh/workspace.json';
import zhVoiceSynth from './locales/zh/voice_synth.json';
import zhVoiceSummary from './locales/zh/voice_summary.json';
import zhChat from './locales/zh/chat.json';
import zhAIVideo from './locales/zh/ai_video.json';
import zhAIMusic from './locales/zh/ai_music.json';
import zhAIImage from './locales/zh/ai_image.json';
import zhAIWriting from './locales/zh/ai_writing.json';
import zhApproval from './locales/zh/approval.json';
import zhAttendance from './locales/zh/attendance.json';
import zhMeeting from './locales/zh/meeting.json';
import zhRecruitment from './locales/zh/recruitment.json';
import zhHardware from './locales/zh/hardware.json';
import zhReport from './locales/zh/report.json';
import zhContacts from './locales/zh/contacts.json';
import zhDrive from './locales/zh/drive.json';
import zhCalendar from './locales/zh/calendar.json';
import zhNotary from './locales/zh/notary.json';
import zhUser from './locales/zh/user.json';
import zhKnowledge from './locales/zh/knowledge.json';
import zhOrders from './locales/zh/orders.json';
import zhCommunity from './locales/zh/community.json';
import zhCourse from './locales/zh/course.json';
import zhEnterprise from './locales/zh/enterprise.json';
import zhVip from './locales/zh/vip.json';
import zhChannels from './locales/zh/channels.json';
import zhErrors from './locales/zh/errors.json';

import enCommon from './locales/en/common.json';
import enAgents from './locales/en/agents.json';
import enAuth from './locales/en/auth.json';
import enShopping from './locales/en/shopping.json';
import enCommons from './locales/en/commons.json';
import enWorkspace from './locales/en/workspace.json';
import enVoiceSynth from './locales/en/voice_synth.json';
import enVoiceSummary from './locales/en/voice_summary.json';
import enChat from './locales/en/chat.json';
import enAIVideo from './locales/en/ai_video.json';
import enAIMusic from './locales/en/ai_music.json';
import enAIImage from './locales/en/ai_image.json';
import enAIWriting from './locales/en/ai_writing.json';
import enApproval from './locales/en/approval.json';
import enAttendance from './locales/en/attendance.json';
import enMeeting from './locales/en/meeting.json';
import enRecruitment from './locales/en/recruitment.json';
import enHardware from './locales/en/hardware.json';
import enReport from './locales/en/report.json';
import enContacts from './locales/en/contacts.json';
import enDrive from './locales/en/drive.json';
import enCalendar from './locales/en/calendar.json';
import enNotary from './locales/en/notary.json';
import enUser from './locales/en/user.json';
import enKnowledge from './locales/en/knowledge.json';
import enOrders from './locales/en/orders.json';
import enCommunity from './locales/en/community.json';
import enCourse from './locales/en/course.json';
import enEnterprise from './locales/en/enterprise.json';
import enVip from './locales/en/vip.json';
import enChannels from './locales/en/channels.json';
import enErrors from './locales/en/errors.json';

const resources = {
  zh: {
    translation: {
      common: zhCommon,
      workspace: zhWorkspace,
      chat: zhChat,
      voice_synth: zhVoiceSynth,
      voice_summary: zhVoiceSummary,
      ai_video: zhAIVideo,
      ai_music: zhAIMusic,
      ai_image: zhAIImage,
      ai_writing: zhAIWriting,
      approval: zhApproval,
      attendance: zhAttendance,
      meeting: zhMeeting,
      recruitment: zhRecruitment,
      hardware: zhHardware,
      report: zhReport,
      contacts: zhContacts,
      drive: zhDrive,
      calendar: zhCalendar,
      notary: zhNotary,
      user: zhUser,
      agents: zhAgents,
      auth: zhAuth,
      shopping: zhShopping,
      commons: zhCommons,
      knowledge: zhKnowledge,
      orders: zhOrders,
      community: zhCommunity,
      course: zhCourse,
      enterprise: zhEnterprise,
      vip: zhVip,
      channels: zhChannels,
      errors: zhErrors
    }
  },
  en: {
    translation: {
      common: enCommon,
      workspace: enWorkspace,
      chat: enChat,
      voice_synth: enVoiceSynth,
      voice_summary: enVoiceSummary,
      ai_video: enAIVideo,
      ai_music: enAIMusic,      
      ai_image: enAIImage,
      ai_writing: enAIWriting,
      approval: enApproval,
      attendance: enAttendance,
      meeting: enMeeting,
      recruitment: enRecruitment,
      hardware: enHardware,
      report: enReport,
      contacts: enContacts,
      drive: enDrive,
      calendar: enCalendar,
      notary: enNotary,
      user: enUser,
      agents: enAgents,
      auth: enAuth,
      shopping: enShopping,
      commons: enCommons,
      knowledge: enKnowledge,
      orders: enOrders,
      community: enCommunity,
      course: enCourse,
      enterprise: enEnterprise,
      vip: enVip,
      channels: enChannels,
      errors: enErrors
    }
  }
};

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources,
    fallbackLng: 'zh',
    interpolation: {
      escapeValue: false, // not needed for react as it escapes by default
    }
  });

export default i18n;
