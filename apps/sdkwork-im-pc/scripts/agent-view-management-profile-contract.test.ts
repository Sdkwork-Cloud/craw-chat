import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import ts from 'typescript';

const sourceText = readFileSync(
  '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/pages/AgentView.tsx',
  'utf8',
);
const sourceFile = ts.createSourceFile(
  'AgentView.tsx',
  sourceText,
  ts.ScriptTarget.Latest,
  true,
  ts.ScriptKind.TSX,
);

function findVariableInitializer(name: string): ts.Expression | undefined {
  let match: ts.Expression | undefined;
  const visit = (node: ts.Node): void => {
    if (
      ts.isVariableDeclaration(node)
      && ts.isIdentifier(node.name)
      && node.name.text === name
      && node.initializer
    ) {
      match = node.initializer;
      return;
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return match;
}

function collectReturnedObjectKeys(expression: ts.Expression | undefined): Set<string> {
  const keys = new Set<string>();
  if (!expression || !ts.isArrowFunction(expression)) {
    return keys;
  }
  const body = expression.body;
  if (!ts.isParenthesizedExpression(body) || !ts.isObjectLiteralExpression(body.expression)) {
    return keys;
  }
  for (const property of body.expression.properties) {
    if (ts.isPropertyAssignment(property) && ts.isIdentifier(property.name)) {
      keys.add(property.name.text);
    }
    if (ts.isShorthandPropertyAssignment(property)) {
      keys.add(property.name.text);
    }
  }
  return keys;
}

const agentKeys = collectReturnedObjectKeys(findVariableInitializer('mapToAgent'));

assert.ok(
  agentKeys.has('welcomeMessage'),
  'AgentView.mapToAgent must preserve welcomeMessage from managed agent profile',
);

console.log('sdkwork im pc agent view management profile contract passed.');
