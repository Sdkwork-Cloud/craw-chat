import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const sdkRoot = path.dirname(fileURLToPath(import.meta.url));
const imRoot = path.join(sdkRoot, 'sdkwork-im-sdk');
const fragmentPath = path.join(imRoot, 'openapi', 'im-spaces-paths.fragment.yaml');
const schemasFragmentPath = path.join(imRoot, 'openapi', 'im-spaces-schemas.fragment.yaml');
const HTTP_METHODS = ['get', 'post', 'put', 'patch', 'delete', 'head', 'options'];

const ACTION_SUMMARIES = {
  accept: 'Accept',
  create: 'Create',
  delete: 'Delete',
  get: 'Get',
  list: 'List',
  revoke: 'Revoke',
  update: 'Update',
};

function summaryFromOperationId(operationId) {
  if (typeof operationId !== 'string' || operationId.length === 0) {
    return 'Space operation';
  }
  const parts = operationId.split('.');
  const actionKey = parts[parts.length - 1] ?? 'operation';
  const action = ACTION_SUMMARIES[actionKey] ?? actionKey;
  const subject = parts
    .slice(0, -1)
    .join(' ')
    .replace(/([a-z])([A-Z])/g, '$1 $2')
    .replace(/\s+/g, ' ')
    .trim();
  return subject.length > 0 ? `${action} ${subject}` : action;
}

function ensureSpaceOperationSummaries(authority) {
  for (const pathItem of Object.values(authority.paths ?? {})) {
    if (!pathItem || typeof pathItem !== 'object') {
      continue;
    }
    for (const method of HTTP_METHODS) {
      const operation = pathItem[method];
      if (!operation || typeof operation !== 'object' || operation.summary) {
        continue;
      }
      operation.summary = summaryFromOperationId(operation.operationId);
    }
  }
}

export function mergeImSpacesOpenApiFragments(authority, yaml) {
  const pathFragment = yaml.load(fs.readFileSync(fragmentPath, 'utf8'));
  const schemaFragment = yaml.load(fs.readFileSync(schemasFragmentPath, 'utf8'));

  authority.tags = authority.tags ?? [];
  if (!authority.tags.some((tag) => tag.name === 'spaces')) {
    authority.tags.push({ name: 'spaces' });
  }

  authority.paths = authority.paths ?? {};
  for (const [pathKey, pathItem] of Object.entries(pathFragment)) {
    authority.paths[pathKey] = pathItem;
  }

  authority.components = authority.components ?? {};
  authority.components.parameters = {
    ...authority.components.parameters,
    ...schemaFragment.parameters,
  };
  authority.components.schemas = {
    ...authority.components.schemas,
    ...schemaFragment.schemas,
  };

  ensureSpaceOperationSummaries(authority);

  return authority;
}
