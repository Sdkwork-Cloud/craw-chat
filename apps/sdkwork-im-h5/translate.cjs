const fs = require('fs');
const path = require('path');

function hash(str) {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = ((hash << 5) - hash) + str.charCodeAt(i);
    hash |= 0; 
  }
  return Math.abs(hash).toString(16);
}

const files = [
  'packages/sdkwork-im-h5-user/src/pages/OtherSettingsSubPages.tsx',
  'packages/sdkwork-im-h5-user/src/pages/MyWorksPage.tsx',
  'packages/sdkwork-im-h5-user/src/pages/MyVoices.tsx',
  'packages/sdkwork-im-h5-user/src/components/moments/MomentItemCard.tsx',
  'packages/sdkwork-im-h5-user/src/components/voice/CreateVoicePreviewStep.tsx',
];

const localePath = 'packages/sdkwork-im-h5-commons/src/locales/en/user.json';
let enDict = {};
if (fs.existsSync(localePath)) {
  enDict = JSON.parse(fs.readFileSync(localePath, 'utf8'));
}

files.forEach(file => {
  if (!fs.existsSync(file)) return;
  let code = fs.readFileSync(file, 'utf8');
  let hasChange = false;

  // Process JSX Text like: >中文<
  code = code.replace(/>([^<]*?[\u4e00-\u9fa5]+[^<]*?)</g, (match, text) => {
    const trimmed = text.trim();
    if (!trimmed || trimmed.includes('{') || trimmed.includes('}')) return match; 
    const key = `auto_${hash(trimmed)}`;
    enDict[key] = trimmed;
    hasChange = true;
    const before = text.substring(0, text.indexOf(trimmed));
    const after = text.substring(text.indexOf(trimmed) + trimmed.length);
    return `>${before}{t('user.auto_${hash(trimmed)}', \`${trimmed}\`)}${after}<`;
  });

  // Process JSX props like label="中文" or placeholder="中文"
  code = code.replace(/([a-zA-Z]+)="([^"]*?[\u4e00-\u9fa5]+[^"]*?)"/g, (match, propName, text) => {
    if (propName === 'className' || propName === 'src' || propName === 'href' || propName === 'id') return match;
    const key = `auto_prop_${hash(text)}`;
    enDict[key] = text;
    hasChange = true;
    return `${propName}={t('user.${key}', "${text}")}`;
  });

  if (hasChange && !code.includes('useTranslation')) {
    code = `import { useTranslation } from "react-i18next";\n` + code;
  }
  
  if (hasChange) {
    fs.writeFileSync(file, code);
  }
});

fs.writeFileSync(localePath, JSON.stringify(enDict, null, 2));
console.log('done');
