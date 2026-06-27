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

function findVariableInitializer(name: string): ts.Expression | undefined {
  let match: ts.Expression | undefined;
  const visit = (node: ts.Node): void => {
    if (
      ts.isVariableDeclaration(node) &&
      ts.isIdentifier(node.name) &&
      node.name.text === name &&
      node.initializer
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

function collectCallNames(): Set<string> {
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

function hasUseStateTupleBinding(stateName: string, setterName: string): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (
      ts.isVariableDeclaration(node) &&
      ts.isArrayBindingPattern(node.name) &&
      node.name.elements.length >= 2 &&
      ts.isBindingElement(node.name.elements[0]) &&
      ts.isBindingElement(node.name.elements[1]) &&
      ts.isIdentifier(node.name.elements[0].name) &&
      ts.isIdentifier(node.name.elements[1].name) &&
      node.name.elements[0].name.text === stateName &&
      node.name.elements[1].name.text === setterName &&
      node.initializer &&
      ts.isCallExpression(node.initializer) &&
      ts.isIdentifier(node.initializer.expression) &&
      node.initializer.expression.text === 'useState'
    ) {
      found = true;
      return;
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return found;
}

function buildCurrentAgentTypeUsesState(): boolean {
  const expression = findVariableInitializer('buildCurrentAgentConfig');
  if (!expression || !ts.isArrowFunction(expression)) {
    return false;
  }
  const body = expression.body;
  if (!ts.isParenthesizedExpression(body) || !ts.isObjectLiteralExpression(body.expression)) {
    return false;
  }
  const typeProperty = body.expression.properties.find(
    (property): property is ts.PropertyAssignment =>
      ts.isPropertyAssignment(property) &&
      ts.isIdentifier(property.name) &&
      property.name.text === 'type',
  );
  return Boolean(
    typeProperty &&
      ts.isIdentifier(typeProperty.initializer) &&
      typeProperty.initializer.text === 'agentType',
  );
}

function hasCallWithArgument(
  callName: string,
  predicate: (argument: ts.Expression) => boolean,
): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (
      ts.isCallExpression(node) &&
      ts.isIdentifier(node.expression) &&
      node.expression.text === callName &&
      node.arguments[0] &&
      predicate(node.arguments[0])
    ) {
      found = true;
      return;
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return found;
}

function hasLoadedAgentTestMessageRestore(): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (ts.isIfStatement(node)) {
      const calls = new Set<string>();
      const collectCalls = (child: ts.Node): void => {
        if (ts.isCallExpression(child) && ts.isIdentifier(child.expression)) {
          calls.add(child.expression.text);
        }
        ts.forEachChild(child, collectCalls);
      };
      collectCalls(node.thenStatement);
      if (calls.has('setWelcomeMessage') && calls.has('setTestMessages')) {
        found = true;
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return found;
}

const configKeys = collectReturnedObjectKeys(findVariableInitializer('buildCurrentAgentConfig'));
for (const key of [
  'debugMode',
  'jsonMode',
  'memoryEnabled',
  'model',
  'temperature',
  'suggestedPrompts',
  'voiceIds',
  'toolIds',
  'skillIds',
]) {
  assert.ok(configKeys.has(key), `buildCurrentAgentConfig must persist ${key}`);
}

const callNames = collectCallNames();
for (const setter of [
  'setDebugMode',
  'setJsonMode',
  'setMemoryEnabled',
  'setModel',
  'setTemperature',
  'setSuggestedPrompts',
  'setSelectedVoiceIds',
  'setSelectedKnowledgeIds',
  'setSelectedToolIds',
    'setSelectedSkillIds',
    'setWelcomeMessage',
    'setAgentType',
  ]) {
  assert.ok(callNames.has(setter), `CreateAgentView must restore ${setter} from loaded agents`);
}

assert.ok(
  hasUseStateTupleBinding('agentType', 'setAgentType'),
  'CreateAgentView must keep agent type as hidden state so modal-selected deployment mode survives editing',
);

assert.ok(
  buildCurrentAgentTypeUsesState(),
  'buildCurrentAgentConfig must persist the current agentType instead of hard-coding normal',
);

assert.ok(
  hasCallWithArgument(
    'setAgentType',
    (argument) =>
      ts.isPropertyAccessExpression(argument) &&
      ts.isIdentifier(argument.expression) &&
      argument.expression.text === 'agent' &&
      argument.name.text === 'type',
  ),
  'CreateAgentView must restore agent type from the loaded agent',
);

assert.ok(
  hasCallWithArgument(
    'setAgentType',
    (argument) => ts.isStringLiteral(argument) && argument.text === 'normal',
  ),
  'CreateAgentView must reset agent type to normal in create mode',
);

assert.ok(
  hasLoadedAgentTestMessageRestore(),
  'CreateAgentView must restore the preview welcome message when loading an existing agent',
);

console.log('sdkwork im pc create agent view management profile contract passed.');
