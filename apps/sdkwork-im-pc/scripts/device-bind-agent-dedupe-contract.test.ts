import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const sourceText = readFileSync(
  './packages/sdkwork-im-pc-devices/src/components/BindAgentModal.tsx',
  'utf8',
);

assert.match(
  sourceText,
  /function mergeUniqueAgents\(/u,
  'BindAgentModal must have an explicit mergeUniqueAgents helper for my + marketplace source dedupe',
);
assert.match(
  sourceText,
  /mergeUniqueAgents\(minePage\.items, marketPage\.items\)/u,
  'BindAgentModal must merge mine agents before marketplace agents through mergeUniqueAgents so private records win',
);
assert.doesNotMatch(
  sourceText,
  /setAgents\(\[\.\.\./u,
  'BindAgentModal must not directly spread agent arrays because duplicate ids render duplicate selectable cards',
);

console.log('sdkwork im pc device bind agent dedupe contract passed.');
