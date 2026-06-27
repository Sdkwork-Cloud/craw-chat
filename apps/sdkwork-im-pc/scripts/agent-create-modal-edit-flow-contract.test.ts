import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import ts from 'typescript';

const modalSourceText = readFileSync(
  '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/components/CreateAgentModal.tsx',
  'utf8',
);
const layoutSourceText = readFileSync(
  '../../../sdkwork-im/apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx',
  'utf8',
);
const defaultsSourceText = readFileSync(
  '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/components/AgentDefaults.ts',
  'utf8',
);

const modalSourceFile = ts.createSourceFile(
  'CreateAgentModal.tsx',
  modalSourceText,
  ts.ScriptTarget.Latest,
  true,
  ts.ScriptKind.TSX,
);
const layoutSourceFile = ts.createSourceFile(
  'ChatLayout.tsx',
  layoutSourceText,
  ts.ScriptTarget.Latest,
  true,
  ts.ScriptKind.TSX,
);
const defaultsSourceFile = ts.createSourceFile(
  'AgentDefaults.ts',
  defaultsSourceText,
  ts.ScriptTarget.Latest,
  true,
  ts.ScriptKind.TS,
);

function collectCallNames(sourceFile: ts.SourceFile): Set<string> {
  const calls = new Set<string>();
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression)) {
      calls.add(node.expression.text);
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return calls;
}

function hasCreateAgentResultIdFlow(sourceFile: ts.SourceFile): boolean {
  let hasFlow = false;
  const visit = (node: ts.Node): void => {
    if (ts.isTryStatement(node)) {
      const createdAgentVariables = new Set<string>();
      const visitTryBlock = (child: ts.Node): void => {
        if (
          ts.isVariableDeclaration(child)
          && child.initializer
          && ts.isAwaitExpression(child.initializer)
          && ts.isCallExpression(child.initializer.expression)
          && ts.isIdentifier(child.name)
        ) {
          const expression = child.initializer.expression.expression;
          if (ts.isPropertyAccessExpression(expression) && expression.name.text === 'createAgent') {
            createdAgentVariables.add(child.name.text);
          }
        }
        if (ts.isCallExpression(child) && ts.isIdentifier(child.expression) && child.expression.text === 'onSuccess') {
          const [argument] = child.arguments;
          if (
            argument
            && ts.isPropertyAccessExpression(argument)
            && ts.isIdentifier(argument.expression)
            && createdAgentVariables.has(argument.expression.text)
            && argument.name.text === 'id'
          ) {
            hasFlow = true;
          }
        }
        ts.forEachChild(child, visitTryBlock);
      };
      visitTryBlock(node.tryBlock);
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return hasFlow;
}

function hasObjectProperty(
  node: ts.ObjectLiteralExpression,
  propertyName: string,
  predicate?: (property: ts.PropertyAssignment | ts.ShorthandPropertyAssignment) => boolean,
): boolean {
  return node.properties.some((property) => {
    const isMatchingProperty =
      (
        ts.isPropertyAssignment(property) &&
        ts.isIdentifier(property.name) &&
        property.name.text === propertyName
      ) ||
      (
        ts.isShorthandPropertyAssignment(property) &&
        property.name.text === propertyName
      );
    return isMatchingProperty && (!predicate || predicate(property));
  });
}

function hasObjectSpread(node: ts.ObjectLiteralExpression, expressionName: string): boolean {
  return node.properties.some(
    (property) =>
      ts.isSpreadAssignment(property) &&
      ts.isIdentifier(property.expression) &&
      property.expression.text === expressionName,
  );
}

function findExportedObjectLiteral(sourceFile: ts.SourceFile, name: string): ts.ObjectLiteralExpression | undefined {
  let match: ts.ObjectLiteralExpression | undefined;
  const visit = (node: ts.Node): void => {
    if (
      ts.isVariableDeclaration(node) &&
      ts.isIdentifier(node.name) &&
      node.name.text === name &&
      node.initializer
    ) {
      const initializer = ts.isSatisfiesExpression(node.initializer)
        ? node.initializer.expression
        : node.initializer;
      if (ts.isObjectLiteralExpression(initializer)) {
        match = initializer;
        return;
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return match;
}

function hasCreateAgentDefaults(sourceFile: ts.SourceFile): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (
      ts.isCallExpression(node) &&
      ts.isPropertyAccessExpression(node.expression) &&
      node.expression.name.text === 'createAgent' &&
      node.arguments[0] &&
      ts.isObjectLiteralExpression(node.arguments[0])
    ) {
      const body = node.arguments[0];
      found = hasObjectSpread(body, 'DEFAULT_AGENT_CONFIG');
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return found;
}

function defaultConfigHasManagedProfileFields(sourceFile: ts.SourceFile): boolean {
  const defaults = findExportedObjectLiteral(sourceFile, 'DEFAULT_AGENT_CONFIG');
  if (!defaults) {
    return false;
  }
  return [
    'debugMode',
    'jsonMode',
    'memoryEnabled',
    'model',
    'temperature',
    'suggestedPrompts',
    'voiceIds',
    'toolIds',
    'skillIds',
    'knowledgeBaseIds',
    'welcomeMessage',
  ].every((propertyName) => hasObjectProperty(defaults, propertyName));
}

assert.ok(
  hasCreateAgentResultIdFlow(modalSourceFile),
  'CreateAgentModal must pass the created agent id to onSuccess',
);

assert.ok(
  modalSourceText.includes('./AgentDefaults') &&
    modalSourceText.includes('DEFAULT_AGENT_CONFIG') &&
    hasCreateAgentDefaults(modalSourceFile) &&
    defaultConfigHasManagedProfileFields(defaultsSourceFile),
  'CreateAgentModal must create agents with the same default management profile used by the full editor',
);

const layoutCalls = collectCallNames(layoutSourceFile);
assert.ok(
  layoutCalls.has('setEditAgentId'),
  'ChatLayout must open the full agent editor with the created agent id',
);

console.log('sdkwork im pc create agent modal edit flow contract passed.');
