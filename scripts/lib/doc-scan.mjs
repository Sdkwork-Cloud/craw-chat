import fs from 'node:fs';
import path from 'node:path';

export function collectMarkdownFiles(repoRoot, relativeDir) {
  const absoluteDir = path.join(repoRoot, relativeDir);
  const results = [];
  for (const entry of fs.readdirSync(absoluteDir, { withFileTypes: true })) {
    const relativePath = path.join(relativeDir, entry.name);
    if (entry.isDirectory()) {
      results.push(...collectMarkdownFiles(repoRoot, relativePath));
      continue;
    }
    if (entry.isFile() && entry.name.endsWith('.md')) {
      results.push(relativePath.replace(/\\/g, '/'));
    }
  }
  return results.sort();
}

export function readRepoMarkdown(repoRoot, relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

export function writeRepoMarkdown(repoRoot, relativePath, source) {
  fs.writeFileSync(path.join(repoRoot, relativePath), source, 'utf8');
}
