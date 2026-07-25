const fs = require('fs');

function walk(dir) {
  let results = [];
  const list = fs.readdirSync(dir);
  list.forEach(function(file) {
    file = dir + '/' + file;
    const stat = fs.statSync(file);
    if (stat && stat.isDirectory()) { 
      results = results.concat(walk(file));
    } else if (file.endsWith('.tsx') || file.endsWith('.ts')) {
      results.push(file);
    }
  });
  return results;
}

function checkFile(filePath) {
  if (!fs.existsSync(filePath)) return;
  const lines = fs.readFileSync(filePath, 'utf8').split('\n');
  let found = false;
  lines.forEach((line, i) => {
    // Only flag lines that have Chinese characters but NO `t("` or `t('`
    if (/[\u4e00-\u9fa5]/.test(line) && !line.includes('t("') && !line.includes("t('")) {
      console.log(`${filePath}:${i+1}:${line.trim()}`);
      found = true;
    }
  });
  if (found) console.log('---');
}

const files = [
  ...walk('packages/sdkwork-im-h5-user/src/pages'),
  ...walk('packages/sdkwork-im-h5-chat/src'),
  ...walk('packages/sdkwork-im-h5-contacts/src'),
  ...walk('packages/sdkwork-im-h5-user/src/services')
];

files.forEach(checkFile);
