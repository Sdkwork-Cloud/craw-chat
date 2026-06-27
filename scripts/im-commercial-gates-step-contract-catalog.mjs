import { createStrictContractCatalog } from './strict-contract-catalog.mjs';

export const IM_COMMERCIAL_GATES_WORKFLOW_STEP_CONTRACTS = [
  {
    id: 'setup-node-v5-before-governance-runner',
    patternSource: String.raw`Setup Node[\s\S]*?uses:\s*actions\/setup-node@v5[\s\S]*?Run commercial gates governance node tests[\s\S]*?run:\s*node scripts\/run-commercial-gates-governance-node-tests\.mjs`,
    message: 'im commercial gates workflow must provision Node via actions/setup-node@v5 before delegating governance node tests to the repository-owned runner',
  },
  {
    id: 'pc-auth-appbase-ui-contract-uses-sdkwork-im-pc-workspace',
    patternSource: String.raw`Test Sdkwork IM PC auth appbase UI contract[\s\S]*?working-directory:\s*apps\/sdkwork-im-pc[\s\S]*?run:\s*node scripts\/auth-appbase-ui-contract\.test\.mjs`,
    message: 'im commercial gates workflow must execute the Sdkwork IM PC auth appbase UI contract inside apps/sdkwork-im-pc',
  },
  {
    id: 'pc-domain-app-sdk-auth-runtime-contract-uses-sdkwork-im-pc-workspace',
    patternSource: String.raw`Test Sdkwork IM PC domain app SDK auth runtime contract[\s\S]*?working-directory:\s*apps\/sdkwork-im-pc[\s\S]*?run:\s*node scripts\/domain-app-sdk-auth-runtime-contract\.test\.mjs`,
    message: 'im commercial gates workflow must execute the Sdkwork IM PC domain app SDK auth runtime contract inside apps/sdkwork-im-pc',
  },
  {
    id: 'pc-notary-app-sdk-contract-uses-sdkwork-im-pc-workspace',
    patternSource: String.raw`Test Sdkwork IM PC notary app SDK integration contract[\s\S]*?working-directory:\s*apps\/sdkwork-im-pc[\s\S]*?run:\s*node scripts\/notary-app-sdk-integration-contract\.test\.mjs`,
    message: 'im commercial gates workflow must execute the Sdkwork IM PC notary app SDK integration contract inside apps/sdkwork-im-pc',
  },
  {
    id: 'pc-drive-app-sdk-contract-uses-sdkwork-im-pc-workspace',
    patternSource: String.raw`Test Sdkwork IM PC drive app SDK integration contract[\s\S]*?working-directory:\s*apps\/sdkwork-im-pc[\s\S]*?run:\s*node scripts\/drive-app-sdk-integration-contract\.test\.mjs`,
    message: 'im commercial gates workflow must execute the Sdkwork IM PC drive app SDK integration contract inside apps/sdkwork-im-pc',
  },
  {
    id: 'pc-knowledgebase-app-sdk-contract-uses-sdkwork-im-pc-workspace',
    patternSource: String.raw`Test Sdkwork IM PC knowledgebase app SDK integration contract[\s\S]*?working-directory:\s*apps\/sdkwork-im-pc[\s\S]*?run:\s*node scripts\/knowledgebase-app-sdk-integration-contract\.test\.mjs`,
    message: 'im commercial gates workflow must execute the Sdkwork IM PC knowledgebase app SDK integration contract inside apps/sdkwork-im-pc',
  },
  {
    id: 'pc-commerce-app-sdk-contract-uses-sdkwork-im-pc-workspace',
    patternSource: String.raw`Test Sdkwork IM PC commerce app SDK integration contract[\s\S]*?working-directory:\s*apps\/sdkwork-im-pc[\s\S]*?run:\s*node scripts\/commerce-app-sdk-integration-contract\.test\.mjs`,
    message: 'im commercial gates workflow must execute the Sdkwork IM PC commerce app SDK integration contract inside apps/sdkwork-im-pc',
  },
  {
    id: 'pc-mail-app-sdk-contract-uses-sdkwork-im-pc-workspace',
    patternSource: String.raw`Test Sdkwork IM PC mail app SDK integration contract[\s\S]*?working-directory:\s*apps\/sdkwork-im-pc[\s\S]*?run:\s*node scripts\/mail-app-sdk-integration-contract\.test\.mjs`,
    message: 'im commercial gates workflow must execute the Sdkwork IM PC mail app SDK integration contract inside apps/sdkwork-im-pc',
  },
  {
    id: 'pc-community-app-sdk-contract-uses-sdkwork-im-pc-workspace',
    patternSource: String.raw`Test Sdkwork IM PC community app SDK integration contract[\s\S]*?working-directory:\s*apps\/sdkwork-im-pc[\s\S]*?run:\s*node scripts\/community-app-sdk-integration-contract\.test\.mjs`,
    message: 'im commercial gates workflow must execute the Sdkwork IM PC community app SDK integration contract inside apps/sdkwork-im-pc',
  },
  {
    id: 'pc-course-app-sdk-contract-uses-sdkwork-im-pc-workspace',
    patternSource: String.raw`Test Sdkwork IM PC course app SDK integration contract[\s\S]*?working-directory:\s*apps\/sdkwork-im-pc[\s\S]*?run:\s*node scripts\/course-app-sdk-integration-contract\.test\.mjs`,
    message: 'im commercial gates workflow must execute the Sdkwork IM PC course app SDK integration contract inside apps/sdkwork-im-pc',
  },
  {
    id: 'pc-aiot-devices-sdk-contract-uses-sdkwork-im-pc-workspace',
    patternSource: String.raw`Test Sdkwork IM PC AIoT devices SDK integration contract[\s\S]*?working-directory:\s*apps\/sdkwork-im-pc[\s\S]*?run:\s*node scripts\/sdkwork-chat-pc-aiot-devices-contract\.test\.mjs`,
    message: 'im commercial gates workflow must execute the Sdkwork IM PC AIoT devices SDK integration contract inside apps/sdkwork-im-pc',
  },
];

const imCommercialGatesWorkflowStepContractCatalog = createStrictContractCatalog({
  contracts: IM_COMMERCIAL_GATES_WORKFLOW_STEP_CONTRACTS,
  duplicateIdMessagePrefix: 'duplicate im commercial gates workflow step contract id',
  missingIdMessagePrefix: 'missing im commercial gates workflow step contract',
});

export function listImCommercialGatesWorkflowStepContracts() {
  return imCommercialGatesWorkflowStepContractCatalog.list();
}

export function findImCommercialGatesWorkflowStepContract(contractId) {
  return imCommercialGatesWorkflowStepContractCatalog.find(contractId);
}

export function listImCommercialGatesWorkflowStepContractsByIds(contractIds = []) {
  return imCommercialGatesWorkflowStepContractCatalog.listByIds(contractIds);
}
