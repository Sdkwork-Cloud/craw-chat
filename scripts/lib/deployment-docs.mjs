import fs from 'node:fs';
import path from 'node:path';

export function resolveDeploymentDocsRoot(repoRoot) {
  const docsRoot = path.join(repoRoot, 'docs');
  for (const entry of fs.readdirSync(docsRoot, { withFileTypes: true })) {
    if (!entry.isDirectory()) {
      continue;
    }
    const marker = path.join(docsRoot, entry.name, 'postgresql-database-configuration.md');
    if (fs.existsSync(marker)) {
      return path.join(docsRoot, entry.name);
    }
  }
  throw new Error('deployment documentation directory must include postgresql-database-configuration.md');
}

export function readDeploymentDoc(repoRoot, fileName) {
  const absolutePath = path.join(resolveDeploymentDocsRoot(repoRoot), fileName);
  if (!fs.existsSync(absolutePath)) {
    throw new Error(`deployment docs must include ${fileName}`);
  }
  return fs.readFileSync(absolutePath, 'utf8');
}

export function readDeploymentDocsMatching(repoRoot, pattern) {
  const deploymentDir = resolveDeploymentDocsRoot(repoRoot);
  return fs
    .readdirSync(deploymentDir)
    .filter((fileName) => pattern.test(fileName))
    .map((fileName) => fs.readFileSync(path.join(deploymentDir, fileName), 'utf8'));
}

export const DEPLOYMENT_DOC_FILES = {
  postgresqlIndex: 'postgresql-database-configuration.md',
  ubuntuWslGuide: 'Ubuntu与WSL-PostgreSQL初始化建库授权手册.md',
  developmentGuide: '开发环境PostgreSQL数据库配置教程.md',
  productionGuide: '线上环境PostgreSQL数据库配置教程.md',
  databaseNaming: 'database-table-naming-standard.md',
};
