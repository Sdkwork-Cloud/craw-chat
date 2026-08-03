
const http = require('node:http');
const probe = () => new Promise((resolve) => {
  const req = http.get({ host: '127.0.0.1', port: 18079, path: '/healthz', timeout: 3000 }, (r) => { r.resume(); resolve(r.statusCode); });
  req.on('error', () => resolve(0));
  req.on('timeout', () => { req.destroy(); resolve(0); });
});
(async () => {
  let state = 'running';
  for (let i = 0; i < 25 && state !== 'new-up'; i++) {
    await new Promise(r => setTimeout(r, 10000));
    const s = await probe();
    if (state === 'running' && s === 0) { state = 'down'; console.log('old down ~' + ((i + 1) * 10) + 's'); }
    else if (state === 'down' && s === 200) { state = 'new-up'; console.log('NEW gateway up ~' + ((i + 1) * 10) + 's'); }
  }
  console.log('final:', state);
  process.exit(0);
})();
