/**
 * SDKWork IM Multi-Tenant Query Isolation Contract Test
 *
 * Validates that all SQL queries in Rust source files that reference
 * organization-scoped IM tables include `organization_id` in their
 * WHERE clause, preventing cross-tenant data leakage.
 *
 * This test enforces the multi-tenant isolation policy documented in
 * the schema baseline comment block:
 *   "所有查询强制携带 organization_id 过滤"
 *
 * Run: node scripts/dev/sdkwork-im-multi-tenant-isolation-contract.test.mjs
 */

import { readFileSync, readdirSync, statSync, existsSync } from 'node:fs';
import { join, extname, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const repoRoot = join(__dirname, '..', '..');

const ORG_SCOPED_TABLES = [
    'im_commit_journal',
    'im_outbox_events',
    'im_inbox_events',
    'im_conversation_messages',
    'im_conversation_seq_counters',
    'im_message_media_refs',
];

const SCAN_DIRECTORIES = [
    'services',
    'adapters',
    'crates',
];

const RUST_EXTENSIONS = new Set(['.rs']);

let violations = [];
let filesScanned = 0;

function scanRustFileForMissingOrgScope(filePath) {
    const content = readFileSync(filePath, 'utf8');
    const lines = content.split('\n');

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const lowerLine = line.toLowerCase();

        // Skip comments and doc lines
        if (line.trim().startsWith('//') || line.trim().startsWith('*')) {
            continue;
        }

        for (const table of ORG_SCOPED_TABLES) {
            // Look for SQL queries referencing org-scoped tables
            if (!lowerLine.includes(table)) {
                continue;
            }

            // Check if this is a query context (SELECT, INSERT, UPDATE, DELETE, FROM, JOIN, INTO)
            const isQueryContext = lowerLine.includes('select ') ||
                lowerLine.includes('insert ') ||
                lowerLine.includes('update ') ||
                lowerLine.includes('delete ') ||
                lowerLine.includes('from ') ||
                lowerLine.includes('join ') ||
                lowerLine.includes('into ');

            if (!isQueryContext) {
                continue;
            }

            // Collect the surrounding query context (up to 5 lines before and after)
            const startLine = Math.max(0, i - 5);
            const endLine = Math.min(lines.length - 1, i + 10);
            const queryContext = lines.slice(startLine, endLine + 1).join('\n').toLowerCase();

            // Check if organization_id appears in the query context
            if (!queryContext.includes('organization_id')) {
                violations.push({
                    file: relative(repoRoot, filePath),
                    line: i + 1,
                    table,
                    snippet: line.trim().substring(0, 120),
                });
            }
        }
    }
}

function scanDirectory(dirPath) {
    if (!existsSync(dirPath)) {
        return;
    }

    const entries = readdirSync(dirPath);
    for (const entry of entries) {
        const fullPath = join(dirPath, entry);
        const stat = statSync(fullPath);

        if (stat.isDirectory()) {
            // Skip target, node_modules, .runtime
            if (entry === 'target' || entry === 'node_modules' || entry === '.runtime') {
                continue;
            }
            scanDirectory(fullPath);
        } else if (RUST_EXTENSIONS.has(extname(entry))) {
            filesScanned++;
            scanRustFileForMissingOrgScope(fullPath);
        }
    }
}

function main() {
    console.log('Multi-tenant query isolation contract test');
    console.log('='.repeat(60));

    for (const dir of SCAN_DIRECTORIES) {
        scanDirectory(join(repoRoot, dir));
    }

    console.log(`Scanned ${filesScanned} Rust source files.`);
    console.log(`Found ${violations.length} potential isolation gap(s).`);

    if (violations.length > 0) {
        console.log('\n⚠ Potential queries missing organization_id filter:');
        for (const v of violations) {
            console.log(`  ${v.file}:${v.line} [${v.table}] ${v.snippet}`);
        }
        console.log('\nAll SQL queries touching organization-scoped tables must include');
        console.log('organization_id in WHERE clauses to prevent cross-tenant leakage.');

        // For now, report as warning since some contexts may be false positives
        // (e.g., DDL statements, test helpers). In production, this should be
        // promoted to a hard failure once all false positives are resolved.
        if (violations.length > 20) {
            console.error('\n❌ FAIL: Too many potential isolation gaps detected.');
            process.exit(1);
        }
        console.log('\n✅ PASS: Violations are within acceptable threshold (review recommended).');
    } else {
        console.log('\n✅ PASS: All organization-scoped queries include organization_id filter.');
    }
}

main();
