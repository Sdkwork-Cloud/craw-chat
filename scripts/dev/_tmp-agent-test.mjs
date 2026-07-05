import fs from 'node:fs';
const s = fs.readFileSync('../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/pages/CreateAgentView.tsx','utf8');
const r = /agentService\.getAgent\s*\(\s*initialAgentId\s*\)\.then\s*\(\s*\(\s*agent\s*\)/u;
console.log(r.test(s));
console.log(s.includes('agentService.getAgent(initialAgentId)'));
