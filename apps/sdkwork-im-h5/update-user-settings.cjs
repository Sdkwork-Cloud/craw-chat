const fs = require('fs');

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

// AccountSecuritySubPages.tsx
replaceFile('packages/sdkwork-im-h5-user/src/pages/AccountSecuritySubPages.tsx', {
  '\"请输入完整信息\"': 't(\"user:account_sec.enter_full_info\", \"请输入完整信息\")',
  '\"手机号已更变\"': 't(\"user:account_sec.phone_changed\", \"手机号已更变\")',
  '\"绑定手机号\"': 't(\"user:account_sec.bind_phone\", \"绑定手机号\")',
  '你的手机号码：+86 138****8888': '{t(\"user:account_sec.your_phone\", \"你的手机号码：+86 138****8888\")}',
  '绑定的手机号可用于登录 sdkwork_im_h5，或找回密码。': '{t(\"user:account_sec.bind_phone_desc\", \"绑定的手机号可用于登录 sdkwork_im_h5，或找回密码。\")}',
  '更换手机号<': '{t(\"user:account_sec.change_phone\", \"更换手机号\")}<',
  '验证新手机号<': '{t(\"user:account_sec.verify_new_phone\", \"验证新手机号\")}<',
  '\"请填写手机号\"': 't(\"user:account_sec.enter_phone\", \"请填写手机号\")',
  '\"验证码\"': 't(\"user:account_sec.verification_code\", \"验证码\")',
  '\"验证码已发送\"': 't(\"user:account_sec.code_sent\", \"验证码已发送\")',
  '获取验证码<': '{t(\"user:account_sec.get_code\", \"获取验证码\")}<',
  '提交<': '{t(\"user:account_sec.submit\", \"提交\")}<',
  '\"设置密码\"': 't(\"user:account_sec.set_password\", \"设置密码\")',
  '\"请填写原密码\"': 't(\"user:account_sec.enter_old_pwd\", \"请填写原密码\")',
  '\"请填写新密码\"': 't(\"user:account_sec.enter_new_pwd\", \"请填写新密码\")',
  '\"请再次填写新密码\"': 't(\"user:account_sec.confirm_new_pwd\", \"请再次填写新密码\")',
  '密码必须包含字母和数字，且长度不少于8位。': '{t(\"user:account_sec.pwd_requirements\", \"密码必须包含字母和数字，且长度不少于8位。\")}',
  '\"操作已执行\"': 't(\"user:account_sec.operation_executed\", \"操作已执行\")',
  '完成<': '{t(\"user:account_sec.done\", \"完成\")}<',
  '\"声音锁\"': 't(\"user:account_sec.voice_lock\", \"声音锁\")',
  '\"登录 sdkwork_im_h5\"': 't(\"user:account_sec.login_sdkwork_im_h5\", \"登录 sdkwork_im_h5\")',
  '开启后，可以使用声音解锁应用或验证身份。': '{t(\"user:account_sec.voice_lock_desc\", \"开启后，可以使用声音解锁应用或验证身份。\")}',
  '\"重设声音锁\"': 't(\"user:account_sec.reset_voice_lock\", \"重设声音锁\")',
  '\"爸爸\"': 't(\"user:account_sec.dad\", \"爸爸\")',
  '\"父亲\"': 't(\"user:account_sec.father\", \"父亲\")',
  '\"李小明\"': 't(\"user:account_sec.friend_name\", \"李小明\")',
  '\"朋友\"': 't(\"user:account_sec.friend\", \"朋友\")',
  '\"应急联系人\"': 't(\"user:account_sec.emergency_contacts\", \"应急联系人\")',
  '在无法登录时，可以联系他们协助恢复账号。': '{t(\"user:account_sec.emergency_desc\", \"在无法登录时，可以联系他们协助恢复账号。\")}',
  '\"添加联系人\"': 't(\"user:account_sec.add_contact\", \"添加联系人\")',
  '\"近期登录设备\"': 't(\"user:account_sec.recent_devices\", \"近期登录设备\")',
  '\"当前设备\"': 't(\"user:account_sec.current_device\", \"当前设备\")',
  '\"2026-07-16 10:00\"': '\"2026-07-16 10:00\"',
  '\"登录时间：\"': 't(\"user:account_sec.login_time\", \"登录时间：\")',
  '\"注销账号\"': 't(\"user:account_sec.delete_account\", \"注销账号\")',
  '注销后，你的账号将被永久删除。': '{t(\"user:account_sec.delete_account_desc\", \"注销后，你的账号将被永久删除。\")}',
  '提交注销申请<': '{t(\"user:account_sec.submit_delete_request\", \"提交注销申请\")}<'
});

// AccountSettings.tsx
replaceFile('packages/sdkwork-im-h5-user/src/pages/settings-pages/AccountSettings.tsx', {
  '\"账号与安全\"': 't(\"user:settings.account_security\", \"账号与安全\")',
  '\"手机号\"': 't(\"user:account_sec.phone\", \"手机号\")',
  '\"微信\"': 't(\"user:account_sec.wechat\", \"微信\")',
  '\"已绑定\"': 't(\"user:account_sec.bound\", \"已绑定\")',
  '\"密码\"': 't(\"user:account_sec.password\", \"密码\")',
  '\"未设置\"': 't(\"user:account_sec.not_set\", \"未设置\")',
  '\"声音锁\"': 't(\"user:account_sec.voice_lock\", \"声音锁\")',
  '\"应急联系人\"': 't(\"user:account_sec.emergency_contacts\", \"应急联系人\")',
  '\"近期登录设备\"': 't(\"user:account_sec.recent_devices\", \"近期登录设备\")',
  '\"注销账号\"': 't(\"user:account_sec.delete_account\", \"注销账号\")'
});

// Settings.tsx
replaceFile('packages/sdkwork-im-h5-user/src/pages/Settings.tsx', {
  '\"设置\"': 't(\"user:settings.title\", \"设置\")',
  '\"账号与安全\"': 't(\"user:settings.account_security\", \"账号与安全\")',
  '\"青少年模式\"': 't(\"user:settings.teen_mode\", \"青少年模式\")',
  '\"关怀模式\"': 't(\"user:settings.care_mode\", \"关怀模式\")',
  '\"新消息通知\"': 't(\"user:settings.message_notifications\", \"新消息通知\")',
  '\"聊天\"': 't(\"user:settings.chat\", \"聊天\")',
  '\"隐私\"': 't(\"user:settings.privacy\", \"隐私\")',
  '\"通用\"': 't(\"user:settings.general\", \"通用\")',
  '\"帮助与反馈\"': 't(\"user:settings.help_feedback\", \"帮助与反馈\")',
  '\"关于 sdkwork_im_h5\"': 't(\"user:settings.about\", \"关于 sdkwork_im_h5\")',
  '\"插件\"': 't(\"user:settings.plugins\", \"插件\")',
  '\"切换账号\"': 't(\"user:settings.switch_account\", \"切换账号\")',
  '\"退出登录\"': 't(\"user:settings.logout\", \"退出登录\")',
  '\"退出登录后将无法收到新消息通知，确认退出？\"': 't(\"user:settings.logout_confirm\", \"退出登录后将无法收到新消息通知，确认退出？\")',
  '\"取消\"': 't(\"user:settings.cancel\", \"取消\")',
  '\"确定\"': 't(\"user:settings.confirm\", \"确定\")'
});

// OtherSettingsSubPages.tsx
replaceFile('packages/sdkwork-im-h5-user/src/pages/OtherSettingsSubPages.tsx', {
  '\"新消息通知\"': 't(\"user:settings.message_notifications\", \"新消息通知\")',
  '\"接收新消息通知\"': 't(\"user:other_settings.recv_new_msg\", \"接收新消息通知\")',
  '\"接收语音和视频通话邀请通知\"': 't(\"user:other_settings.recv_call\", \"接收语音和视频通话邀请通知\")',
  '\"通知显示消息详情\"': 't(\"user:other_settings.show_msg_detail\", \"通知显示消息详情\")',
  '\"声音\"': 't(\"user:other_settings.sound\", \"声音\")',
  '\"震动\"': 't(\"user:other_settings.vibrate\", \"震动\")',
  '\"聊天\"': 't(\"user:settings.chat\", \"聊天\")',
  '\"聊天背景\"': 't(\"user:other_settings.chat_bg\", \"聊天背景\")',
  '\"表情管理\"': 't(\"user:other_settings.emoji_mgr\", \"表情管理\")',
  '\"清空聊天记录\"': 't(\"user:other_settings.clear_history\", \"清空聊天记录\")',
  '\"通用\"': 't(\"user:settings.general\", \"通用\")',
  '\"深色模式\"': 't(\"user:other_settings.dark_mode\", \"深色模式\")',
  '\"多语言\"': 't(\"user:other_settings.multi_language\", \"多语言\")',
  '\"简体中文\"': 't(\"user:other_settings.zh_cn\", \"简体中文\")',
  '\"字体大小\"': 't(\"user:other_settings.font_size\", \"字体大小\")',
  '\"照片、视频、文件和通话\"': 't(\"user:other_settings.media_file_call\", \"照片、视频、文件和通话\")',
  '\"开启横屏模式\"': 't(\"user:other_settings.landscape_mode\", \"开启横屏模式\")',
  '\"存储空间\"': 't(\"user:other_settings.storage_space\", \"存储空间\")',
  '\"插件\"': 't(\"user:settings.plugins\", \"插件\")',
  '看一看<': '{t(\"user:other_settings.look_around\", \"看一看\")}<',
  '发现朋友关注的热点<': '{t(\"user:other_settings.discover_trends\", \"发现朋友关注的热点\")}<',
  '搜一搜<': '{t(\"user:other_settings.search_around\", \"搜一搜\")}<',
  '搜索文章、小程序等<': '{t(\"user:other_settings.search_articles\", \"搜索文章、小程序等\")}<'
});

