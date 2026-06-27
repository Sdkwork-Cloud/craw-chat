import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import ts from 'typescript';

function parseSource(path: string, kind: ts.ScriptKind = ts.ScriptKind.TS): ts.SourceFile {
  return ts.createSourceFile(
    path,
    readFileSync(path, 'utf8'),
    ts.ScriptTarget.Latest,
    true,
    kind,
  );
}

function findInterface(sourceFile: ts.SourceFile, name: string): ts.InterfaceDeclaration | undefined {
  let match: ts.InterfaceDeclaration | undefined;
  const visit = (node: ts.Node): void => {
    if (ts.isInterfaceDeclaration(node) && node.name.text === name) {
      match = node;
      return;
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return match;
}

function interfaceHasProperty(
  sourceFile: ts.SourceFile,
  interfaceName: string,
  propertyName: string,
): boolean {
  return Boolean(
    findInterface(sourceFile, interfaceName)?.members.some(
      (member) => ts.isPropertySignature(member) &&
        ts.isIdentifier(member.name) &&
        member.name.text === propertyName,
    ),
  );
}

function sourceContainsAll(sourceText: string, snippets: string[]): boolean {
  return snippets.every((snippet) => sourceText.includes(snippet));
}

const chatTypeSource = parseSource('../../../sdkwork-im/apps/sdkwork-im-pc/packages/sdkwork-im-pc-types/src/chat.ts');
assert.ok(
  interfaceHasProperty(chatTypeSource, 'Chat', 'welcomeMessage'),
  'Shared Chat view model must expose welcomeMessage for managed agent conversations',
);

const chatServiceText = readFileSync(
  '../../../sdkwork-im/apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts',
  'utf8',
);
for (const snippet of [
  "'avatar' | 'name' | 'welcomeMessage'",
  "welcomeMessage: viewState?.welcomeMessage",
  "welcomeMessage: agent.welcomeMessage",
]) {
  assert.ok(
    chatServiceText.includes(snippet),
    `ChatService must preserve agent welcome message through ${snippet}`,
  );
}

const chatWindowText = readFileSync(
  '../../../sdkwork-im/apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatWindow.tsx',
  'utf8',
);
assert.ok(
  sourceContainsAll(chatWindowText, [
    'agentWelcomeMessages',
    'chat.welcomeMessage',
    'agentSenderProfiles',
    'fallbackMessages={displayWelcomeMessages}',
    'senderProfiles={displaySenderProfiles}',
  ]),
  'ChatWindow must render managed agent welcomeMessage through MessageList fallback messages',
);

console.log('sdkwork im pc agent chat welcome message contract passed.');
