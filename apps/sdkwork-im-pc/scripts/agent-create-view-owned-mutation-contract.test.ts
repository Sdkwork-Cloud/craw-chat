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

function hasSetDraftIdInitialAgentIdCall(): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (
      ts.isCallExpression(node) &&
      ts.isIdentifier(node.expression) &&
      node.expression.text === 'setDraftId' &&
      node.arguments[0] &&
      ts.isIdentifier(node.arguments[0]) &&
      node.arguments[0].text === 'initialAgentId'
    ) {
      found = true;
      return;
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return found;
}

function hasResolverGuard(): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (
      ts.isVariableDeclaration(node) &&
      ts.isIdentifier(node.name) &&
      node.name.text === 'resolveMutableAgentId' &&
      node.initializer &&
      ts.isArrowFunction(node.initializer)
    ) {
      const bodyText = node.initializer.body.getText(sourceFile);
      found =
        bodyText.includes('draftId') &&
        bodyText.includes('initialAgentId') &&
        bodyText.includes('throw new Error');
      return;
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return found;
}

assert.ok(
  !hasSetDraftIdInitialAgentIdCall(),
  'CreateAgentView must not trust initialAgentId as mutable draftId before owned-agent lookup succeeds',
);

assert.ok(
  hasResolverGuard(),
  'CreateAgentView must guard save, publish, preview, and prompt optimization behind a confirmed owned draftId in edit mode',
);

assert.ok(
  !sourceText.includes('draftId ?? initialAgentId'),
  'CreateAgentView mutations must not fall back to route initialAgentId when ownership was not confirmed',
);

console.log('sdkwork im pc create agent owned mutation contract passed.');
