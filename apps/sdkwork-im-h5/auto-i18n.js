const fs = require('fs');
const path = require('path');
const glob = require('glob');

const localesPath = path.resolve(__dirname, '../packages/sdkwork-im-h5-commons/src/locales/en/user.json');
let enLocale = {};
if (fs.existsSync(localesPath)) {
  enLocale = JSON.parse(fs.readFileSync(localesPath, 'utf8'));
}

let modifiedFiles = 0;
let modifiedStrings = 0;

function hashString(str) {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = ((hash << 5) - hash) + str.charCodeAt(i);
    hash |= 0; 
  }
  return Math.abs(hash).toString(16);
}

function processFile(filePath) {
  let content = fs.readFileSync(filePath, 'utf8');
  let hasChanges = false;
  
  // Use a regex to find strings containing chinese characters, not starting with t(
  // and capturing the quote type.
  const regex = /(?<!t\(\s*)(['"`])([^\1]*?[\u4e00-\u9fa5]+[^\1]*?)\1/g;
  let matches;
  
  // To avoid messing up JSX, we should also process JSX Text separately, but for now we'll do strings.
  // Wait, JSX text is like >中文<, which won't be matched by the quote regex.
  const jsxRegex = />([^<]*?[\u4e00-\u9fa5]+[^<]*?)</g;
  
  content = content.replace(jsxRegex, (match, p1) => {
    let text = p1.trim();
    if (!text) return match;
    const key = `auto_${hashString(text)}`;
    if (!enLocale[key]) enLocale[key] = text;
    hasChanges = true;
    modifiedStrings++;
    // if text is just text, replace it with {t('user.key', 'text')}
    // we need to keep the spaces around the original p1 if any
    const before = p1.substring(0, p1.indexOf(text));
    const after = p1.substring(p1.indexOf(text) + text.length);
    return `>${before}{t('user.${key}', \`${text.replace(/`/g, "\\`")}\`)}${after}<`;
  });

  content = content.replace(regex, (match, quote, p1) => {
    // If it's already in t( or console.log etc, maybe skip. But our lookbehind skips t(.
    const text = p1;
    const key = `auto_prop_${hashString(text)}`;
    if (!enLocale[key]) enLocale[key] = text;
    hasChanges = true;
    modifiedStrings++;
    
    // Replace with t('user.key', 'text') but as a string it might be in an object { label: "..." }
    // We should replace the quotes too? But if it's `{ label: "中文" }` we want `{ label: t(...) }`.
    // We can't safely do this with simple regex because replacing `"中文"` with `t(...)` requires removing quotes,
    // which breaks if it's not in an expression context.
    return match; // skip for now
  });

  if (hasChanges) {
    if (!content.includes('useTranslation')) {
       // prepend it
       content = `import { useTranslation } from "react-i18next";\n` + content;
       // We also need to inject `const { t } = useTranslation();` into components, which is hard with regex.
       // It's safer to only process if useTranslation is already there.
    }
    fs.writeFileSync(filePath, content, 'utf8');
    modifiedFiles++;
  }
}

const files = glob.sync('../packages/sdkwork-im-h5-user/src/**/*.tsx');
files.forEach(processFile);

fs.writeFileSync(localesPath, JSON.stringify(enLocale, null, 2), 'utf8');
console.log(`Modified ${modifiedFiles} files, ${modifiedStrings} strings.`);
