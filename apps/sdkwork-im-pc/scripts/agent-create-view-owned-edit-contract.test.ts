import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import ts from 'typescript';

const sourceText = readFileSync(
  '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/pages/CreateAgentView.tsx',
  'utf8',
);
const sourceFile = ts.createSourceFile(
  'CreateAgentView.tsx',
  sourceText,
  ts.ScriptTarget.Latest,
  true,
  ts.ScriptKind.TSX,
);

function collectAgentServiceMethodCalls(): string[] {
  const calls: string[] = [];
  const visit = (node: ts.Node): void => {
    if (
      ts.isCallExpression(node) &&
      ts.isPropertyAccessExpression(node.expression) &&
      ts.isIdentifier(node.expression.expression) &&
      node.expression.expression.text === 'agentService'
    ) {
      calls.push(node.expression.name.text);
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return calls;
}

const agentServiceCalls = collectAgentServiceMethodCalls();

assert.ok(
  agentServiceCalls.includes('getAgents'),
  'CreateAgentView edit mode must load the editable target from the current user owned agent list',
);

assert.ok(
  !agentServiceCalls.includes('getMarketAgents'),
  'CreateAgentView edit mode must not load market agents as editable targets',
);

assert.ok(
  !sourceText.includes('...marketAgents') && !sourceText.includes('marketAgents.find'),
  'CreateAgentView must not merge marketplace records into the editable agent lookup',
);

console.log('sdkwork im pc create agent owned edit contract passed.');
