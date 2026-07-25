const fs = require('fs');

function checkFile(filePath) {
  if (!fs.existsSync(filePath)) return;
  const lines = fs.readFileSync(filePath, 'utf8').split('\n');
  let found = false;
  lines.forEach((line, i) => {
    if (/[\u4e00-\u9fa5]/.test(line)) {
      console.log(`${filePath}:${i+1}:${line}`);
      found = true;
    }
  });
  if (found) console.log('---');
}

checkFile('packages/sdkwork-im-h5-user/src/pages/AccountSecuritySubPages.tsx');
