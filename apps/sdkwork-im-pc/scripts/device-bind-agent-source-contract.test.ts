import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import ts from 'typescript';

const sourceText = readFileSync(
  './packages/sdkwork-im-pc-devices/src/components/BindAgentModal.tsx',
  'utf8',
);
const sourceFile = ts.createSourceFile(
  'BindAgentModal.tsx',
  sourceText,
  ts.ScriptTarget.Latest,
  true,
  ts.ScriptKind.TSX,
);

function hasAgentServiceCall(methodName: string): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (
      ts.isCallExpression(node) &&
      ts.isPropertyAccessExpression(node.expression) &&
      ts.isIdentifier(node.expression.expression) &&
      node.expression.expression.text === 'agentService' &&
      node.expression.name.text === methodName
    ) {
      found = true;
      return;
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return found;
}

function hasMergedAgentListStateUpdate(): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (
      ts.isCallExpression(node) &&
      ts.isIdentifier(node.expression) &&
      node.expression.text === 'setAgents' &&
      node.arguments[0] &&
      ts.isCallExpression(node.arguments[0]) &&
      ts.isIdentifier(node.arguments[0].expression) &&
      node.arguments[0].expression.text === 'mergeUniqueAgents' &&
      node.arguments[0].arguments.some((argument) =>
        ts.isIdentifier(argument) &&
        argument.text === 'myAgents',
      ) &&
      node.arguments[0].arguments.some((argument) =>
        ts.isIdentifier(argument) &&
        argument.text === 'marketAgents',
      )
    ) {
      found = true;
      return;
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return found;
}

assert.ok(
  hasAgentServiceCall('getAgents'),
  'BindAgentModal must include my private agents so device binding works before marketplace publication',
);
assert.ok(
  hasAgentServiceCall('getMarketAgents'),
  'BindAgentModal must keep marketplace agents available for device binding',
);
assert.ok(
  hasMergedAgentListStateUpdate(),
  'BindAgentModal must merge and dedupe myAgents and marketAgents into the selectable agent list without changing the UI',
);

console.log('sdkwork im pc device bind agent source contract passed.');
