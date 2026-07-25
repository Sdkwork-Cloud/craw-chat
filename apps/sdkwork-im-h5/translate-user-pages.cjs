const fs = require('fs');
const path = require('path');

function replaceFile(filePath, replacements) {
  if (!fs.existsSync(filePath)) return;
  let content = fs.readFileSync(filePath, 'utf8');

  if (!content.includes('useTranslation')) {
    content = content.replace(/(import .* from ['"]react-router['"];)/, "$1\nimport { useTranslation } from 'react-i18next';");
    content = content.replace(/(export const \w+ = \(\) => {)/, "$1\n  const { t } = useTranslation();");
  }

  for (const [key, val] of Object.entries(replacements)) {
    content = content.split(key).join(val);
  }

  fs.writeFileSync(filePath, content);
}

// HelpAboutSettings.tsx
replaceFile('packages/sdkwork-im-h5-user/src/pages/settings-pages/HelpAboutSettings.tsx', {
  '\"帮助与反馈\"': 't(\"user:help_about.title\", \"帮助与反馈\")',
  '\"常见问题\"': 't(\"user:help_about.faq\", \"常见问题\")',
  '\"意见反馈\"': 't(\"user:help_about.feedback\", \"意见反馈\")',
  '\"关于 sdkwork_im_h5\"': 't(\"user:help_about.about_title\", \"关于 sdkwork_im_h5\")',
  '\"功能介绍\"': 't(\"user:help_about.features\", \"功能介绍\")',
  '\"投诉\"': 't(\"user:help_about.complain\", \"投诉\")',
  '\"软件许可及服务协议\"': 't(\"user:help_about.terms\", \"软件许可及服务协议\")',
  '\"隐私保护指引\"': 't(\"user:help_about.privacy\", \"隐私保护指引\")'
});

// ModeSettings.tsx
replaceFile('packages/sdkwork-im-h5-user/src/pages/settings-pages/ModeSettings.tsx', {
  '\"青少年模式\"': 't(\"user:mode.teen_title\", \"青少年模式\")',
  '青少年模式': '{t(\"user:mode.teen_title\", \"青少年模式\")}',
  '开启后，将限制部分功能的使用，并限制使用时间。开启或关闭都需要输入独立密码。': '{t(\"user:mode.teen_desc\", \"开启后，将限制部分功能的使用，并限制使用时间。开启或关闭都需要输入独立密码。\")}',
  '\"关闭青少年模式\"': 't(\"user:mode.teen_off\", \"关闭青少年模式\")',
  '\"开启青少年模式\"': 't(\"user:mode.teen_on\", \"开启青少年模式\")',
  '\"关怀模式\"': 't(\"user:mode.care_title\", \"关怀模式\")',
  '关怀模式': '{t(\"user:mode.care_title\", \"关怀模式\")}',
  '开启后，文字和按钮将变得更大，色彩更强。': '{t(\"user:mode.care_desc\", \"开启后，文字和按钮将变得更大，色彩更强。\")}',
  '\"关闭关怀模式\"': 't(\"user:mode.care_off\", \"关闭关怀模式\")',
  '\"开启关怀模式\"': 't(\"user:mode.care_on\", \"开启关怀模式\")'
});

// PrivacySettings.tsx
replaceFile('packages/sdkwork-im-h5-user/src/pages/settings-pages/PrivacySettings.tsx', {
  '\"朋友权限\"': 't(\"user:privacy.friend_perms\", \"朋友权限\")',
  '\"通讯录黑名单\"': 't(\"user:privacy.blacklist\", \"通讯录黑名单\")',
  '添加我的方式': '{t(\"user:privacy.add_methods\", \"添加我的方式\")}',
  '\"手机号\"': 't(\"user:privacy.phone\", \"手机号\")',
  '\"微信号\"': 't(\"user:privacy.wechat_id\", \"微信号\")',
  '\"个人信息与权限\"': 't(\"user:privacy.info_perms\", \"个人信息与权限\")',
  '\"系统权限管理\"': 't(\"user:privacy.sys_perms\", \"系统权限管理\")',
  '\"授权管理\"': 't(\"user:privacy.auth_mgr\", \"授权管理\")',
  '\"个性化广告管理\"': 't(\"user:privacy.ads_mgr\", \"个性化广告管理\")',
  '\"个人信息收集清单\"': 't(\"user:privacy.collection_list\", \"个人信息收集清单\")',
  '个人信息收集清单': '{t(\"user:privacy.collection_list\", \"个人信息收集清单\")}',
  '为了向您提供 sdkwork_im_h5 的各项服务，我们需要收集您的以下个人信息：': '{t(\"user:privacy.collection_desc\", \"为了向您提供 sdkwork_im_h5 的各项服务，我们需要收集您的以下个人信息：\")}',
  '\"基本信息\"': 't(\"user:privacy.basic_info\", \"基本信息\")',
  '\"头像、昵称、性别、地区\"': 't(\"user:privacy.basic_info_val\", \"头像、昵称、性别、地区\")',
  '\"设备信息\"': 't(\"user:privacy.device_info\", \"设备信息\")',
  '\"设备型号、操作系统\"': 't(\"user:privacy.device_info_val\", \"设备型号、操作系统\")',
  '\"网络信息\"': 't(\"user:privacy.network_info\", \"网络信息\")',
  '\"IP地址、网络类型\"': 't(\"user:privacy.network_info_val\", \"IP地址、网络类型\")',
  '\"日志信息\"': 't(\"user:privacy.log_info\", \"日志信息\")',
  '\"操作日志、崩溃日志\"': 't(\"user:privacy.log_info_val\", \"操作日志、崩溃日志\")',
  '\"第三方信息共享清单\"': 't(\"user:privacy.sharing_list\", \"第三方信息共享清单\")',
  '第三方信息共享清单': '{t(\"user:privacy.sharing_list\", \"第三方信息共享清单\")}',
  '在为您提供服务时，我们可能会与以下第三方共享您的必要信息：': '{t(\"user:privacy.sharing_desc\", \"在为您提供服务时，我们可能会与以下第三方共享您的必要信息：\")}',
  '\"地图服务提供商\"': 't(\"user:privacy.map_provider\", \"地图服务提供商\")',
  '\"位置信息\"': 't(\"user:privacy.map_provider_val\", \"位置信息\")',
  '\"推送服务提供商\"': 't(\"user:privacy.push_provider\", \"推送服务提供商\")',
  '\"设备标识符\"': 't(\"user:privacy.push_provider_val\", \"设备标识符\")',
  '\"支付服务提供商\"': 't(\"user:privacy.pay_provider\", \"支付服务提供商\")',
  '\"订单信息\"': 't(\"user:privacy.pay_provider_val\", \"订单信息\")'
});

// ProfileMoreSettings.tsx
replaceFile('packages/sdkwork-im-h5-user/src/pages/settings-pages/ProfileMoreSettings.tsx', {
  '\"男\"': 't(\"user:profile.male\", \"男\")',
  '\"女\"': 't(\"user:profile.female\", \"女\")',
  '\"设置性别\"': 't(\"user:profile.set_gender\", \"设置性别\")',
  '男<': '{t(\"user:profile.male\", \"男\")}<',
  '女<': '{t(\"user:profile.female\", \"女\")}<',
  '\"设置地区\"': 't(\"user:profile.set_region\", \"设置地区\")',
  '\"中国大陆\"': 't(\"user:profile.china\", \"中国大陆\")',
  '\"北京\"': 't(\"user:profile.beijing\", \"北京\")',
  '\"个性签名\"': 't(\"user:profile.signature\", \"个性签名\")',
  '\"介绍一下自己吧...\"': 't(\"user:profile.sig_placeholder\", \"介绍一下自己吧...\")',
  '\"保存成功\"': 't(\"user:profile.save_success\", \"保存成功\")',
  '保存<': '{t(\"user:profile.save\", \"保存\")}<'
});

// SystemSettings.tsx
replaceFile('packages/sdkwork-im-h5-user/src/pages/settings-pages/SystemSettings.tsx', {
  '\"通讯录黑名单\"': 't(\"user:system.blacklist\", \"通讯录黑名单\")',
  '你将不会收到列表中联系人的消息，并且他们无法查看你的朋友圈。': '{t(\"user:system.blacklist_desc\", \"你将不会收到列表中联系人的消息，并且他们无法查看你的朋友圈。\")}',
  '\"已移出黑名单\"': 't(\"user:system.removed_from_blacklist\", \"已移出黑名单\")',
  '移除<': '{t(\"user:system.remove\", \"移除\")}<',
  '\"常见问题\"': 't(\"user:system.faq_title\", \"常见问题\")',
  '\"如何找回密码？\"': 't(\"user:system.q1\", \"如何找回密码？\")',
  '您可以在登录页面点击“找回密码”并通过手机验证码重置密码。': '{t(\"user:system.a1\", \"您可以在登录页面点击“找回密码”并通过手机验证码重置密码。\")}',
  '\"如何解冻账号？\"': 't(\"user:system.q2\", \"如何解冻账号？\")',
  '请前往安全中心进行申诉解冻，需要提供实名认证和好友辅助验证。': '{t(\"user:system.a2\", \"请前往安全中心进行申诉解冻，需要提供实名认证和好友辅助验证。\")}',
  '\"如何修改微信号？\"': 't(\"user:system.q3\", \"如何修改微信号？\")',
  '微信号一年只能修改一次，您可以在“个人信息”页点击“微信号”进行修改。': '{t(\"user:system.a3\", \"微信号一年只能修改一次，您可以在“个人信息”页点击“微信号”进行修改。\")}',
  '\"意见反馈\"': 't(\"user:help_about.feedback\", \"意见反馈\")',
  '\"请详细描述您遇到的问题或建议...\"': 't(\"user:system.feedback_placeholder\", \"请详细描述您遇到的问题或建议...\")',
  '\"提交成功，感谢反馈！\"': 't(\"user:system.submit_success\", \"提交成功，感谢反馈！\")',
  '提交<': '{t(\"user:system.submit\", \"提交\")}<',
  '\"功能介绍\"': 't(\"user:system.features_title\", \"功能介绍\")',
  'sdkwork_im_h5 1.0.0 更新日志': '{t(\"user:system.changelog\", \"sdkwork_im_h5 1.0.0 更新日志\")}',
  '1. 全新的 UI 设计': '{t(\"user:system.f1\", \"1. 全新的 UI 设计\")}',
  '2. 支持智能体聊天': '{t(\"user:system.f2\", \"2. 支持智能体聊天\")}',
  '3. 优化了性能和体验': '{t(\"user:system.f3\", \"3. 优化了性能和体验\")}',
  '\`正在投诉 [\${type}]。您可以补充更多信息：\`': '\`\${t(\"user:system.complain_submitting\", \"正在投诉 [{{type}}]。您可以补充更多信息：\", { type })}\`',
  '\"投诉已提交受理\"': 't(\"user:system.complain_success\", \"投诉已提交受理\")',
  '\"投诉\"': 't(\"user:system.complain_title\", \"投诉\")',
  '\"欺诈骗钱\"': 't(\"user:system.fraud\", \"欺诈骗钱\")',
  '\"色情暴力\"': 't(\"user:system.porn_violence\", \"色情暴力\")',
  '\"政治谣言\"': 't(\"user:system.rumor\", \"政治谣言\")',
  '\"软件许可及服务协议\"': 't(\"user:system.terms_title\", \"软件许可及服务协议\")',
  '欢迎使用 sdkwork_im_h5！': '{t(\"user:system.welcome\", \"欢迎使用 sdkwork_im_h5！\")}',
  '在使用本软件前，请您务必仔细阅读并透彻理解本协议...': '{t(\"user:system.terms_desc\", \"在使用本软件前，请您务必仔细阅读并透彻理解本协议...\")}',
  '\"隐私保护指引\"': 't(\"user:system.privacy_title\", \"隐私保护指引\")',
  '我们非常重视您的隐私保护。': '{t(\"user:system.privacy_welcome\", \"我们非常重视您的隐私保护。\")}',
  '本指引将向您说明我们如何收集、使用、存储和共享您的个人信息...': '{t(\"user:system.privacy_desc\", \"本指引将向您说明我们如何收集、使用、存储和共享您的个人信息...\")}',
  '\"管理聊天记录\"': 't(\"user:system.storage\", \"管理聊天记录\")',
  '\"工作群\"': 't(\"user:system.work_group\", \"工作群\")'
});

