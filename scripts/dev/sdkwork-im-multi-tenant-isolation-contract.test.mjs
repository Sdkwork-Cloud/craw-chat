/**
 * Verifies that executable Rust SQL touching organization-scoped IM tables
 * binds tenant and organization predicates or declares one narrowly validated system
 * maintenance operation.
 */

import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { extname, relative, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const scriptDirectory = fileURLToPath(new URL('.', import.meta.url));
const defaultRepoRoot = resolve(scriptDirectory, '..', '..');
const isolationContractRelativePath = 'specs/organization-isolation.spec.json';
const RUST_EXTENSIONS = new Set(['.rs']);
const CROSS_ORGANIZATION_MARKER = /\/\*\s*sdkwork:cross-organization-operation=([a-z0-9-]+)\s*\*\//iu;
const REQUIRED_AUDIT_FIELDS = ['actor_kind', 'actor_id', 'trace_id', 'outcome'];

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

export function missingCrossOrganizationAuditFields(source, auditEvent) {
  const eventOffset = source.indexOf(auditEvent);
  if (eventOffset < 0) return ['event'];
  const evidenceWindow = source.slice(
    Math.max(0, eventOffset - 256),
    Math.min(source.length, eventOffset + 1_500),
  );
  return REQUIRED_AUDIT_FIELDS.filter(
    (field) => !new RegExp(`\\b${field}\\s*(?:=|,)`, 'u').test(evidenceWindow),
  );
}

function validateEvidenceSources(repoRoot, operation) {
  let typedOperationFound = false;
  for (const sourcePath of operation.authorizationEvidenceSources) {
    const absolutePath = resolve(repoRoot, sourcePath);
    if (!existsSync(absolutePath)) {
      throw new Error(`${operation.id} authorization evidence source does not exist: ${sourcePath}`);
    }
    if (readFileSync(absolutePath, 'utf8').includes(operation.operationRequestType)) {
      typedOperationFound = true;
    }
  }
  if (!typedOperationFound) {
    throw new Error(
      `${operation.id} does not expose typed operation ${operation.operationRequestType} in its authorization evidence`,
    );
  }

  let auditEvidenceFound = false;
  const missingBySource = [];
  for (const sourcePath of operation.auditEvidenceSources) {
    const absolutePath = resolve(repoRoot, sourcePath);
    if (!existsSync(absolutePath)) {
      throw new Error(`${operation.id} audit evidence source does not exist: ${sourcePath}`);
    }
    const missing = missingCrossOrganizationAuditFields(
      readFileSync(absolutePath, 'utf8'),
      operation.auditEvent,
    );
    if (missing.length === 0) auditEvidenceFound = true;
    else missingBySource.push(`${sourcePath} (${missing.join(', ')})`);
  }
  if (!auditEvidenceFound) {
    throw new Error(
      `${operation.id} audit evidence is incomplete: ${missingBySource.join('; ')}`,
    );
  }
}

export function loadIsolationContract(repoRoot = defaultRepoRoot) {
  const contractPath = resolve(repoRoot, isolationContractRelativePath);
  const contract = readJson(contractPath);
  if (contract?.schemaVersion !== 1 || contract?.kind !== 'sdkwork.database.organization-isolation') {
    throw new Error(`${isolationContractRelativePath} has an unsupported schemaVersion or kind`);
  }
  if (!Array.isArray(contract.scopeColumns)
    || !contract.scopeColumns.includes('tenant_id')
    || !contract.scopeColumns.includes('organization_id')) {
    throw new Error(`${isolationContractRelativePath} must declare tenant_id and organization_id`);
  }
  if (!Array.isArray(contract.sourceRoots) || contract.sourceRoots.length === 0) {
    throw new Error(`${isolationContractRelativePath} must declare sourceRoots`);
  }
  if (!Array.isArray(contract.crossOrganizationOperations)) {
    throw new Error(`${isolationContractRelativePath} must declare crossOrganizationOperations`);
  }

  const registryPath = resolve(contractPath, '..', contract.tableRegistry);
  const registry = readJson(registryPath);
  const organizationScopedTables = (registry.tables ?? []).map((entry) => entry.table_name);
  if (organizationScopedTables.length === 0 || organizationScopedTables.some((table) => !table)) {
    throw new Error(`${contract.tableRegistry} does not contain a valid table inventory`);
  }

  const operationIds = new Set();
  for (const operation of contract.crossOrganizationOperations) {
    if (!operation.id || operationIds.has(operation.id)) {
      throw new Error(`${isolationContractRelativePath} contains a missing or duplicate operation id`);
    }
    operationIds.add(operation.id);
    for (const field of [
      'owner',
      'authorizationMode',
      'operationRequestType',
      'auditEvent',
      'sqlOperation',
    ]) {
      if (!operation[field]) throw new Error(`${operation.id} must declare ${field}`);
    }
    for (const field of [
      'allowedSqlSources',
      'authorizationEvidenceSources',
      'auditEvidenceSources',
      'tables',
      'requiredSqlFragments',
    ]) {
      if (!Array.isArray(operation[field]) || operation[field].length === 0) {
        throw new Error(`${operation.id} must declare non-empty ${field}`);
      }
    }
    validateEvidenceSources(repoRoot, operation);
  }

  return { ...contract, organizationScopedTables };
}

const defaultIsolationContract = loadIsolationContract();
export const ORG_SCOPED_TABLES = defaultIsolationContract.organizationScopedTables;

function lineNumberAt(source, offset) {
  return source.slice(0, offset).split('\n').length;
}

function isLineComment(source, offset) {
  const lineStart = source.lastIndexOf('\n', offset - 1) + 1;
  return source.slice(lineStart, offset).trimStart().startsWith('//');
}

function isAssertionString(source, offset) {
  const prefix = source.slice(Math.max(0, offset - 96), offset);
  return /\.(?:contains|starts_with|ends_with)\s*\(\s*$/u.test(prefix);
}

function isExecutableSqlCarrier(source, offset) {
  const prefix = source.slice(Math.max(0, offset - 256), offset);
  return /\b(?:const|static)\s+[A-Z0-9_]*SQL[A-Z0-9_]*\s*:\s*&str\s*=\s*(?:r#{0,16})?$/u.test(prefix)
    || /\blet\s+[a-z0-9_]*sql[a-z0-9_]*\s*(?::[^=]+)?=\s*(?:r#{0,16})?$/iu.test(prefix)
    || /\.(?:query|query_one|query_opt|execute|prepare|batch_execute)\s*\(\s*(?:r#{0,16})?$/u.test(prefix);
}

function normalizeNormalString(body) {
  return body
    .replace(/\\\r?\n\s*/gu, ' ')
    .replace(/\\[rnt]/gu, ' ')
    .replace(/\\"/gu, '"')
    .replace(/\\\\/gu, '\\');
}

export function extractRustStringLiterals(source) {
  const literals = [];
  const masked = [...source];
  const rawPattern = /r(#{0,16})"([\s\S]*?)"\1/gu;
  let match;

  while ((match = rawPattern.exec(source)) !== null) {
    if (!isLineComment(source, match.index) && isExecutableSqlCarrier(source, match.index)) {
      literals.push({
        line: lineNumberAt(source, match.index),
        sql: match[2],
      });
    }
    for (let index = match.index; index < rawPattern.lastIndex; index += 1) {
      if (masked[index] !== '\n' && masked[index] !== '\r') masked[index] = ' ';
    }
  }

  const normalSource = masked.join('');
  const normalPattern = /"((?:\\[\s\S]|[^"\\])*)"/gu;
  while ((match = normalPattern.exec(normalSource)) !== null) {
    if (isLineComment(source, match.index)
      || isAssertionString(source, match.index)
      || !isExecutableSqlCarrier(source, match.index)) continue;
    literals.push({
      line: lineNumberAt(source, match.index),
      sql: normalizeNormalString(source.slice(match.index + 1, normalPattern.lastIndex - 1)),
    });
  }

  return literals.sort((left, right) => left.line - right.line);
}

function normalizedSql(sql) {
  return sql.trim().replace(/\s+/gu, ' ').toLowerCase();
}

function executableSql(sql) {
  return sql.replace(/^\s*(?:\/\*[\s\S]*?\*\/\s*)*/u, '');
}

function sqlOperation(sql) {
  const executable = executableSql(sql);
  const direct = executable.match(/^\s*(select|insert|update|delete)\b/iu)?.[1];
  if (direct) return direct.toLowerCase();
  if (!/^\s*with\b/iu.test(executable)) return null;
  const operations = [...executable.matchAll(/\b(select|insert|update|delete)\b/giu)];
  return operations.at(-1)?.[1]?.toLowerCase() ?? null;
}

function isExecutableSql(sql, operation) {
  if (operation === 'select') return /\bfrom\b/iu.test(sql);
  if (operation === 'insert') return /\binto\b/iu.test(sql) && /\b(values|select)\b/iu.test(sql);
  if (operation === 'update') return /\bset\b/iu.test(sql) && /\bwhere\b/iu.test(sql);
  if (operation === 'delete') return /\bfrom\b/iu.test(sql) && /\bwhere\b/iu.test(sql);
  return false;
}

function hasScopeBinding(sql, operation, table) {
  if (operation === 'insert') {
    const escapedTable = table.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&');
    const columns = sql.match(new RegExp(`\\binsert\\s+into\\s+${escapedTable}\\s*\\(([^)]*)\\)`, 'iu'))?.[1];
    if (columns != null) {
      return /\btenant_id\b/iu.test(columns) && /\borganization_id\b/iu.test(columns);
    }
  }

  const whereOffset = sql.search(/\bwhere\b/iu);
  if (whereOffset < 0) return false;
  const predicate = sql.slice(whereOffset);
  return /\b(?:[a-z_][a-z0-9_]*\.)?tenant_id\s*=\s*\$\d+/iu.test(predicate)
    && /\b(?:[a-z_][a-z0-9_]*\.)?organization_id\s*=\s*\$\d+/iu.test(predicate);
}

function validateCrossOrganizationOperation(sql, sqlOperation, table, file, contract) {
  const marker = sql.match(CROSS_ORGANIZATION_MARKER)?.[1];
  if (!marker) return { allowed: false, reason: 'missing tenant_id or organization_id binding' };

  const normalized = normalizedSql(sql);
  const operation = contract.crossOrganizationOperations.find((entry) => entry.id === marker);
  if (!operation) return { allowed: false, reason: `unknown cross-organization operation ${marker}` };
  const normalizedFile = file.replaceAll('\\', '/');
  if (!operation.allowedSqlSources.includes(normalizedFile)) {
    return { allowed: false, reason: `${marker} is not allowed in ${normalizedFile}` };
  }
  if (!operation.tables.includes(table)) {
    return { allowed: false, reason: `${marker} is not approved for ${table}` };
  }
  if (operation.sqlOperation !== sqlOperation
    || operation.requiredSqlFragments.some((fragment) => !normalized.includes(fragment))) {
    return { allowed: false, reason: `invalid ${marker} SQL shape` };
  }
  return { allowed: true, reason: null };
}

export function findIsolationViolationsInRustSource(
  source,
  file = '<memory>',
  contract = defaultIsolationContract,
) {
  const violations = [];
  for (const literal of extractRustStringLiterals(source)) {
    const operation = sqlOperation(literal.sql);
    if (!operation || !isExecutableSql(literal.sql, operation)) continue;

    const lowerSql = literal.sql.toLowerCase();
    for (const table of contract.organizationScopedTables) {
      if (!new RegExp(`\\b${table}\\b`, 'u').test(lowerSql)) continue;
      const marker = literal.sql.match(CROSS_ORGANIZATION_MARKER);
      if (!marker && hasScopeBinding(literal.sql, operation, table)) continue;

      const crossOrganization = marker
        ? validateCrossOrganizationOperation(literal.sql, operation, table, file, contract)
        : { allowed: false, reason: 'missing tenant_id or organization_id binding' };
      if (crossOrganization.allowed) continue;
      violations.push({
        file,
        line: literal.line,
        operation,
        reason: crossOrganization.reason,
        table,
      });
    }
  }
  return violations;
}

function scanDirectory(repoRoot, directory, state, contract) {
  if (!existsSync(directory)) return;
  for (const entry of readdirSync(directory)) {
    const fullPath = resolve(directory, entry);
    const stats = statSync(fullPath);
    if (stats.isDirectory()) {
      if (entry !== 'target' && entry !== 'node_modules') {
        scanDirectory(repoRoot, fullPath, state, contract);
      }
      continue;
    }
    if (!RUST_EXTENSIONS.has(extname(entry))) continue;
    state.filesScanned += 1;
    const file = relative(repoRoot, fullPath);
    state.violations.push(
      ...findIsolationViolationsInRustSource(readFileSync(fullPath, 'utf8'), file, contract),
    );
  }
}

export function scanRepository(repoRoot = defaultRepoRoot) {
  const contract = loadIsolationContract(repoRoot);
  const state = { filesScanned: 0, violations: [] };
  for (const directory of contract.sourceRoots) {
    scanDirectory(repoRoot, resolve(repoRoot, directory), state, contract);
  }
  return state;
}

export function main(repoRoot = defaultRepoRoot) {
  console.log('Multi-tenant query isolation contract test');
  console.log('='.repeat(60));
  const result = scanRepository(repoRoot);
  console.log(`Scanned ${result.filesScanned} Rust source files.`);
  console.log(`Found ${result.violations.length} isolation gap(s).`);

  if (result.violations.length === 0) {
    console.log('PASS: all organization-scoped SQL is isolated or explicitly governed.');
    return;
  }

  for (const violation of result.violations) {
    console.error(
      `  ${violation.file}:${violation.line} [${violation.table}] ${violation.operation}: ${violation.reason}`,
    );
  }
  throw new Error('organization-scoped SQL isolation violations detected');
}

const entryUrl = process.argv[1] ? pathToFileURL(resolve(process.argv[1])).href : null;
if (import.meta.url === entryUrl) main();
