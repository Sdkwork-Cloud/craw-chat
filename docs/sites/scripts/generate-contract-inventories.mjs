import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "yaml";

const currentDir = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(currentDir, "..", "..", "..");
const writeMode = process.argv.includes("--write");
const httpMethods = ["get", "post", "put", "patch", "delete", "options", "head", "trace"];

const apiAuthorities = [
  {
    name: "Open API",
    prefix: "/im/v3/api",
    source: "apis/open-api/im/sdkwork-im-im.openapi.yaml",
    materialized: "sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml",
    sdk: "sdkwork-im-sdk",
  },
  {
    name: "App API",
    prefix: "/app/v3/api",
    source: "apis/app-api/communication/sdkwork-im-app-api.openapi.yaml",
    materialized: "sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.openapi.yaml",
    sdk: "sdkwork-im-app-sdk",
  },
  {
    name: "Backend API",
    prefix: "/backend/v3/api",
    source: "apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml",
    materialized: "sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml",
    sdk: "sdkwork-im-backend-sdk",
  },
];

const databaseRegistryPath = "database/contract/table-registry.json";
const databaseManifestPath = "database/database.manifest.json";
const generatedApiSchemasPath =
  "docs/sites/.vitepress/theme/api-schemas/generated-openapi.ts";
const apiReferenceRoot = path.join(repositoryRoot, "docs", "sites", "api-reference");
const manualApiSchemaRoot = path.join(
  repositoryRoot,
  "docs",
  "sites",
  ".vitepress",
  "theme",
  "api-schemas",
);

function readText(relativePath) {
  return fs.readFileSync(path.join(repositoryRoot, relativePath), "utf8");
}

function markdownCode(value) {
  return `\`${String(value).replaceAll("`", "\\`")}\``;
}

function markdownCell(value) {
  return String(value ?? "-").replaceAll("|", "\\|").replaceAll("\r", " ").replaceAll("\n", " ");
}

function compareOperations(left, right) {
  return (
    left.path.localeCompare(right.path) ||
    httpMethods.indexOf(left.method.toLowerCase()) - httpMethods.indexOf(right.method.toLowerCase())
  );
}

function collectFiles(root, predicate) {
  const files = [];
  for (const entry of fs.readdirSync(root, { withFileTypes: true })) {
    const entryPath = path.join(root, entry.name);
    if (entry.isDirectory()) {
      files.push(...collectFiles(entryPath, predicate));
    } else if (entry.isFile() && predicate(entryPath)) {
      files.push(entryPath);
    }
  }
  return files;
}

function readApiAuthority(authority) {
  const document = parse(readText(authority.source));
  const operations = [];

  for (const [routePath, pathItem] of Object.entries(document.paths ?? {})) {
    for (const method of httpMethods) {
      const operation = pathItem?.[method];
      if (!operation) {
        continue;
      }

      if (!routePath.startsWith(authority.prefix)) {
        throw new Error(`${authority.source}: route ${routePath} is outside ${authority.prefix}`);
      }
      if (!operation.operationId) {
        throw new Error(`${authority.source}: ${method.toUpperCase()} ${routePath} has no operationId`);
      }

      operations.push({
        method: method.toUpperCase(),
        path: routePath,
        operationId: operation.operationId,
      });
    }
  }

  operations.sort(compareOperations);
  const operationIds = new Set();
  for (const operation of operations) {
    if (operationIds.has(operation.operationId)) {
      throw new Error(`${authority.source}: duplicate operationId ${operation.operationId}`);
    }
    operationIds.add(operation.operationId);
  }

  return { ...authority, document, operations };
}

function collectReferencedApiSchemas() {
  const schemaNames = new Set();
  for (const filePath of collectFiles(apiReferenceRoot, (candidate) => candidate.endsWith(".md"))) {
    const source = fs.readFileSync(filePath, "utf8");
    for (const match of source.matchAll(/<ApiSchemaTable schema="([A-Za-z][A-Za-z0-9_]*)"/g)) {
      schemaNames.add(match[1]);
    }
  }
  return [...schemaNames].sort((left, right) => left.localeCompare(right));
}

function collectManualApiSchemaNames() {
  const schemaNames = new Set();
  for (const filePath of collectFiles(
    manualApiSchemaRoot,
    (candidate) =>
      candidate.endsWith(".ts") &&
      !candidate.endsWith("generated-openapi.ts") &&
      !candidate.endsWith("index.ts") &&
      !candidate.endsWith("schema-types.ts"),
  )) {
    const source = fs.readFileSync(filePath, "utf8");
    for (const match of source.matchAll(/^  ([A-Za-z][A-Za-z0-9_]*):\s*\{/gm)) {
      schemaNames.add(match[1]);
    }
  }
  return schemaNames;
}

function localRefName(reference) {
  const prefix = "#/components/schemas/";
  return typeof reference === "string" && reference.startsWith(prefix)
    ? reference.slice(prefix.length)
    : null;
}

function resolveLocalSchema(schema, document) {
  const referenceName = localRefName(schema?.$ref);
  return referenceName ? document.components?.schemas?.[referenceName] ?? schema : schema;
}

function schemaType(schema) {
  if (!schema || typeof schema !== "object") {
    return "unknown";
  }

  const referenceName = localRefName(schema.$ref);
  if (referenceName) {
    return referenceName;
  }
  if (schema.oneOf || schema.anyOf) {
    const variants = schema.oneOf ?? schema.anyOf;
    return [...new Set(variants.map(schemaType))].join(" | ");
  }
  if (schema.type === "array") {
    return `${schemaType(schema.items)}[]`;
  }
  if (Array.isArray(schema.type)) {
    return schema.type.map((type) => (type === "null" ? "null" : schemaType({ ...schema, type }))).join(" | ");
  }
  if (schema.type === "object" || schema.properties || schema.additionalProperties) {
    if (schema.additionalProperties && typeof schema.additionalProperties === "object") {
      return `Record<string, ${schemaType(schema.additionalProperties)}>`;
    }
    return "object";
  }
  if (schema.type) {
    return schema.format ? `${schema.type} (${schema.format})` : schema.type;
  }
  if (schema.enum?.length) {
    return schema.enum.map((value) => JSON.stringify(value)).join(" | ");
  }
  return "unknown";
}

function schemaDescription(schema) {
  const details = [];
  if (schema?.description) {
    details.push(String(schema.description).replaceAll(/\s+/g, " ").trim());
  }
  if (schema?.enum?.length) {
    details.push(`Allowed values: ${schema.enum.map((value) => JSON.stringify(value)).join(", ")}.`);
  }
  if (schema?.default !== undefined) {
    details.push(`Default: ${JSON.stringify(schema.default)}.`);
  }
  return details.join(" ") || "Defined by the authored OpenAPI contract.";
}

function objectShape(schema, document, seen = new Set()) {
  if (!schema || typeof schema !== "object") {
    return { properties: {}, required: new Set() };
  }

  const referenceName = localRefName(schema.$ref);
  if (referenceName) {
    if (seen.has(referenceName)) {
      return { properties: {}, required: new Set() };
    }
    return objectShape(resolveLocalSchema(schema, document), document, new Set([...seen, referenceName]));
  }

  const properties = { ...(schema.properties ?? {}) };
  const required = new Set(schema.required ?? []);
  for (const member of schema.allOf ?? []) {
    const memberShape = objectShape(member, document, seen);
    Object.assign(properties, memberShape.properties);
    for (const fieldName of memberShape.required) {
      required.add(fieldName);
    }
  }
  return { properties, required };
}

function renderSchemaField(name, schema, required, document, depth = 0) {
  const field = {
    name,
    type: schemaType(schema),
    description: schemaDescription(schema),
    required,
  };

  if (depth >= 2 || schema?.$ref) {
    return field;
  }

  const nestedSchema = schema?.type === "array" ? schema.items : schema;
  const shape = objectShape(nestedSchema, document);
  const nestedEntries = Object.entries(shape.properties);
  if (nestedEntries.length > 0) {
    field.children = nestedEntries.map(([fieldName, propertySchema]) =>
      renderSchemaField(fieldName, propertySchema, shape.required.has(fieldName), document, depth + 1),
    );
  }
  return field;
}

function renderSchemaDefinition(schema, document) {
  const shape = objectShape(schema, document);
  const entries = Object.entries(shape.properties);
  if (entries.length === 0) {
    return {
      fields: [renderSchemaField("value", schema, true, document)],
    };
  }
  return {
    fields: entries.map(([fieldName, propertySchema]) =>
      renderSchemaField(fieldName, propertySchema, shape.required.has(fieldName), document),
    ),
  };
}

function renderGeneratedApiSchemas(authorities) {
  const definitionsByName = new Map();
  for (const authority of authorities) {
    for (const [name, schema] of Object.entries(authority.document.components?.schemas ?? {})) {
      const entries = definitionsByName.get(name) ?? [];
      entries.push({ authority, schema, serialized: JSON.stringify(schema) });
      definitionsByName.set(name, entries);
    }
  }

  const manualSchemaNames = collectManualApiSchemaNames();
  const generatedSchemas = {};
  for (const name of collectReferencedApiSchemas()) {
    const definitions = definitionsByName.get(name) ?? [];
    const distinctDefinitions = new Map(
      definitions.map((definition) => [definition.serialized, definition]),
    );

    if (distinctDefinitions.size === 0) {
      if (!manualSchemaNames.has(name)) {
        throw new Error(`API docs reference unknown schema ${name}`);
      }
      continue;
    }
    if (distinctDefinitions.size > 1) {
      if (!manualSchemaNames.has(name)) {
        throw new Error(`API docs reference ambiguous cross-surface schema ${name}`);
      }
      continue;
    }

    const [{ authority, schema }] = distinctDefinitions.values();
    generatedSchemas[name] = renderSchemaDefinition(schema, authority.document);
  }

  return [
    "// Generated by docs/sites/scripts/generate-contract-inventories.mjs.",
    "// Do not edit by hand; authored OpenAPI is the contract authority.",
    "",
    'import type { ApiSchemaDefinitionMap } from "./schema-types";',
    "",
    `export const generatedOpenApiSchemas = ${JSON.stringify(generatedSchemas, null, 2)} satisfies ApiSchemaDefinitionMap;`,
    "",
  ].join("\n");
}

function renderApiInventory(authorities) {
  const total = authorities.reduce((sum, authority) => sum + authority.operations.length, 0);
  const lines = [
    "# SDKWork IM HTTP API Inventory",
    "",
    "Status: active",
    "Owner: `im-platform`",
    "Generated: yes",
    "Generator: `docs/sites/scripts/generate-contract-inventories.mjs`",
    "Specs: `API_SPEC.md`, `DOCUMENTATION_SPEC.md`",
    "",
    "This inventory contains only HTTP APIs owned by this repository. Sibling platform and product",
    "dependencies mounted by a gateway are intentionally excluded. Authored OpenAPI under `apis/` is",
    "the contract authority; SDK-family OpenAPI under `sdks/` is a deterministic materialization.",
    "",
    "## Surface Summary",
    "",
    "| Surface | Prefix | Operations | Authored authority | SDK authority | SDK family |",
    "| --- | --- | ---: | --- | --- | --- |",
    ...authorities.map(
      (authority) =>
        `| ${authority.name} | ${markdownCode(authority.prefix)} | ${authority.operations.length} | ${markdownCode(authority.source)} | ${markdownCode(authority.materialized)} | ${markdownCode(authority.sdk)} |`,
    ),
    `| **Total** | - | **${total}** | - | - | - |`,
    "",
    "## Operation Inventory",
    "",
    "Each row is extracted from the authored OpenAPI `paths` object. Method, path, and `operationId`",
    "are public contract identifiers and must change at the OpenAPI source before this file is regenerated.",
    "",
  ];

  for (const authority of authorities) {
    lines.push(`### ${authority.name} (${authority.operations.length})`, "");
    lines.push("| Method | Path | operationId |", "| --- | --- | --- |");
    for (const operation of authority.operations) {
      lines.push(
        `| ${markdownCode(operation.method)} | ${markdownCode(operation.path)} | ${markdownCode(operation.operationId)} |`,
      );
    }
    lines.push("");
  }

  lines.push(
    "## Regeneration And Verification",
    "",
    "```bash",
    "node docs/sites/scripts/generate-contract-inventories.mjs --write",
    "node docs/sites/scripts/generate-contract-inventories.mjs --check",
    "pnpm test:apis-authority-standard",
    "```",
    "",
  );

  return lines.join("\n");
}

function readDatabaseRegistry() {
  const registry = JSON.parse(readText(databaseRegistryPath));
  const manifest = JSON.parse(readText(databaseManifestPath));
  const tables = [...(registry.tables ?? [])].sort((left, right) =>
    left.table_name.localeCompare(right.table_name),
  );

  for (const table of tables) {
    if (!table.table_name?.startsWith("im_")) {
      throw new Error(`${databaseRegistryPath}: non-IM table ${table.table_name ?? "<missing>"}`);
    }
    for (const requiredField of ["bounded_context", "table_profile", "write_owner", "migration"]) {
      if (!table[requiredField]) {
        throw new Error(`${databaseRegistryPath}: ${table.table_name} has no ${requiredField}`);
      }
    }
  }

  return { manifest, registry, tables };
}

function renderDatabaseInventory({ manifest, registry, tables }) {
  const contexts = new Map();
  for (const table of tables) {
    const contextTables = contexts.get(table.bounded_context) ?? [];
    contextTables.push(table);
    contexts.set(table.bounded_context, contextTables);
  }

  const lines = [
    "# SDKWork IM Database Inventory",
    "",
    "Status: active",
    "Owner: `im-platform`",
    "Generated: yes",
    "Generator: `docs/sites/scripts/generate-contract-inventories.mjs`",
    `Source contract: ${markdownCode(databaseRegistryPath)}`,
    "Specs: `DATABASE_SPEC.md`, `DOCUMENTATION_SPEC.md`",
    "",
    "This inventory contains only tables owned by SDKWork IM. IAM, Agents, Drive, Knowledgebase, RTC,",
    "and other sibling databases are external dependencies and are intentionally excluded.",
    "",
    "## Persistence Authority",
    "",
    "- PostgreSQL is the durable IM authority for normalized Conversation, Message, Member, ReadCursor,",
    "  social, realtime, signaling, and operational state.",
    "- `im_commit_journal` is immutable audit/integration evidence, not a source for rebuilding current state.",
    "- `im_outbox_events` and `im_inbox_events` provide transactional integration delivery.",
    "- A business mutation, its journal evidence, and required outbox event commit in one transaction.",
    "- Current state is read from typed normalized tables. No second Message timeline or persisted read-model",
    "  authority is allowed.",
    "- Cross-domain identifiers are opaque references without physical foreign keys to sibling databases.",
    "",
    "## Registry Summary",
    "",
    `- Schema version: ${markdownCode(registry.schemaVersion)}`,
    `- Module: ${markdownCode(manifest.moduleId)}`,
    `- Contract version: ${markdownCode(manifest.contractVersion)}`,
    `- Lifecycle strategy: ${markdownCode(manifest.baselineStrategy)}`,
    `- Registered IM tables: ${tables.length}`,
    `- Runtime engines: ${(manifest.engines ?? []).map(markdownCode).join(", ")}`,
    "",
    "## Table Inventory",
    "",
  ];

  for (const [boundedContext, contextTables] of [...contexts.entries()].sort(([left], [right]) =>
    left.localeCompare(right),
  )) {
    lines.push(`### ${boundedContext} (${contextTables.length})`, "");
    lines.push(
      "| Table | Profile | Write owner | Authority role | Migration / DDL source |",
      "| --- | --- | --- | --- | --- |",
    );
    for (const table of contextTables) {
      lines.push(
        `| ${markdownCode(table.table_name)} | ${markdownCode(table.table_profile)} | ${markdownCode(table.write_owner)} | ${table.system_of_record ? "system of record" : "owned relation / operational state"} | ${markdownCode(markdownCell(table.migration))} |`,
      );
    }
    lines.push("");
  }

  lines.push(
    "## Contract Boundaries",
    "",
    "Field definitions, indexes, constraints, retention, and migration ordering remain authoritative in",
    "the registry-linked DDL and migration sources. The domain invariants are narrowed by",
    "`specs/IM_DOMAIN_AND_PERSISTENCE_SPEC.md`; this generated file is a complete discovery inventory,",
    "not a second schema definition.",
    "",
    "## Regeneration And Verification",
    "",
    "```bash",
    "node docs/sites/scripts/generate-contract-inventories.mjs --write",
    "node docs/sites/scripts/generate-contract-inventories.mjs --check",
    "pnpm test:database-naming-standard",
    "pnpm test:database-framework-standard",
    "```",
    "",
  );

  return lines.join("\n");
}

function synchronize(relativePath, expectedContent) {
  const absolutePath = path.join(repositoryRoot, relativePath);
  const currentContent = fs.existsSync(absolutePath) ? fs.readFileSync(absolutePath, "utf8") : null;

  if (currentContent === expectedContent) {
    return false;
  }
  if (!writeMode) {
    throw new Error(`${relativePath} is out of date; run this generator with --write`);
  }

  fs.writeFileSync(absolutePath, expectedContent, "utf8");
  return true;
}

const authorities = apiAuthorities.map(readApiAuthority);
const changedFiles = [];

if (synchronize("docs/api-reference.md", renderApiInventory(authorities))) {
  changedFiles.push("docs/api-reference.md");
}
if (synchronize("docs/database-design.md", renderDatabaseInventory(readDatabaseRegistry()))) {
  changedFiles.push("docs/database-design.md");
}
if (synchronize(generatedApiSchemasPath, renderGeneratedApiSchemas(authorities))) {
  changedFiles.push(generatedApiSchemasPath);
}

const mode = writeMode ? "Generated" : "Verified";
const operationCount = authorities.reduce((sum, authority) => sum + authority.operations.length, 0);
console.log(
  `${mode} ${operationCount} HTTP operations and ${readDatabaseRegistry().tables.length} IM tables${
    changedFiles.length > 0 ? `; updated ${changedFiles.join(", ")}` : ""
  }.`,
);
