import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  groupedPages,
  markdownPathFor,
  operationMarkdownPath,
} from "../.vitepress/api-reference-sidebar.mjs";

const currentDir = path.dirname(fileURLToPath(import.meta.url));
const docsRoot = path.resolve(currentDir, "..");
const operationsRoot = path.join(docsRoot, "api-reference", "operations");
const domainOverviewLinks = {
  "IM Standard API": "/api-reference/im-api",
  "App API": "/api-reference/app-api",
  "Backend API": "/api-reference/backend-api",
};

function sourceGroupFor(pageLink) {
  for (const group of groupedPages) {
    for (const page of group.pages) {
      if (page.link === pageLink) {
        return { domain: group.text, page: page.text };
      }
    }
  }

  return { domain: "API Reference", page: pageLink };
}

function operationBlocks(pageLink) {
  const content = fs.readFileSync(markdownPathFor(pageLink), "utf8");
  const matches = [
    ...content.matchAll(
      /<a id="([^"]+)"><\/a>\s*<section class="api-op">\s*## `([^`]+)`\s*([\s\S]*?)<\/section>/g,
    ),
  ];

  return matches.map((match) => ({
    anchor: match[1],
    operationTitle: match[2],
    body: match[3].trim(),
  }));
}

function operationFileContent(pageLink, operationTitle, body) {
  const { domain, page } = sourceGroupFor(pageLink);
  const cleanBody = body.replace(/^\s+/, "");
  const domainOverviewLink = domainOverviewLinks[domain] ?? "/api-reference/index";

  return `# \`${operationTitle}\`

<p class="api-page-intro">
  Exact request and response contract for <strong>${page}</strong> in the <strong>${domain}</strong>.
</p>

<div class="api-link-list">
  <a href="${pageLink}"><code>${page}</code> Return to the group page for workflow context and related operations</a>
  <a href="${domainOverviewLink}"><code>${domain}</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

${cleanBody}

</section>
`;
}

fs.rmSync(operationsRoot, { recursive: true, force: true });

let generatedCount = 0;

for (const group of groupedPages) {
  for (const page of group.pages) {
    for (const operation of operationBlocks(page.link)) {
      const outputPath = operationMarkdownPath(page.link, operation.anchor);
      fs.mkdirSync(path.dirname(outputPath), { recursive: true });
      fs.writeFileSync(
        outputPath,
        operationFileContent(page.link, operation.operationTitle, operation.body),
      );
      generatedCount += 1;
    }
  }
}

console.log(`Generated ${generatedCount} operation pages.`);
