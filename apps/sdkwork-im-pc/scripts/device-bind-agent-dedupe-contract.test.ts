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

function hasFunctionDeclaration(functionName: string): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (
      ts.isFunctionDeclaration(node) &&
      node.name?.text === functionName
    ) {
      found = true;
      return;
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return found;
}

function hasSetAgentsFromMergedSources(): boolean {
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
      node.arguments[0].arguments.length === 2 &&
      node.arguments[0].arguments.every((argument, index) =>
        ts.isIdentifier(argument) &&
        argument.text === (index === 0 ? 'myAgents' : 'marketAgents'),
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

function hasDirectSpreadMergeInSetAgents(): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (
      ts.isCallExpression(node) &&
      ts.isIdentifier(node.expression) &&
      node.expression.text === 'setAgents' &&
      node.arguments[0] &&
      ts.isArrayLiteralExpression(node.arguments[0]) &&
      node.arguments[0].elements.some((element) => ts.isSpreadElement(element))
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
  hasFunctionDeclaration('mergeUniqueAgents'),
  'BindAgentModal must have an explicit mergeUniqueAgents helper for my + marketplace source dedupe',
);
assert.ok(
  hasSetAgentsFromMergedSources(),
  'BindAgentModal must merge myAgents before marketAgents through mergeUniqueAgents so private records win',
);
assert.equal(
  hasDirectSpreadMergeInSetAgents(),
  false,
  'BindAgentModal must not directly spread myAgents and marketAgents because duplicate ids render duplicate selectable cards',
);

console.log('sdkwork im pc device bind agent dedupe contract passed.');
