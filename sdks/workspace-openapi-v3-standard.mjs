import { sdkWorkEnvelopeComponentSchemas } from '../../sdkwork-specs/tools/lib/openapi-envelope-schemas.mjs';

const dualTokenSecurity = Object.freeze({
  AuthToken: [],
  AccessToken: [],
});

const httpMethods = new Set([
  'get',
  'put',
  'post',
  'delete',
  'patch',
  'options',
  'head',
  'trace',
]);

function cloneJson(value) {
  return JSON.parse(JSON.stringify(value));
}

function isOperationKey(key) {
  return httpMethods.has(String(key).toLowerCase());
}

function isExplicitAnonymousSecurity(security) {
  return Array.isArray(security) && security.length === 0;
}

function problemDetailSchema() {
  return structuredClone(sdkWorkEnvelopeComponentSchemas.ProblemDetail);
}

function ensureEnvelopeComponentSchemas(schemas) {
  for (const [name, schema] of Object.entries(sdkWorkEnvelopeComponentSchemas)) {
    if (!schemas[name]) {
      schemas[name] = structuredClone(schema);
    }
  }
  schemas.ProblemDetail = problemDetailSchema();
}

function problemJsonContent() {
  return {
    'application/problem+json': {
      schema: {
        $ref: '#/components/schemas/ProblemDetail',
      },
    },
  };
}

function problemResponse(description = 'Problem detail response') {
  return {
    description,
    content: problemJsonContent(),
  };
}

function isErrorStatus(statusCode) {
  const status = Number.parseInt(String(statusCode), 10);
  return Number.isFinite(status) && status >= 400;
}

export function applySdkworkV3OpenApiStandard(document) {
  if (!document || typeof document !== 'object' || Array.isArray(document)) {
    return document;
  }

  document.components = document.components && typeof document.components === 'object'
    ? document.components
    : {};
  document.components.securitySchemes = {
    AuthToken: {
      type: 'http',
      scheme: 'bearer',
      bearerFormat: 'JWT',
    },
    AccessToken: {
      type: 'apiKey',
      in: 'header',
      name: 'Access-Token',
    },
  };
  document.components.schemas = document.components.schemas && typeof document.components.schemas === 'object'
    ? document.components.schemas
    : {};
  ensureEnvelopeComponentSchemas(document.components.schemas);

  document.security = [cloneJson(dualTokenSecurity)];

  for (const pathItem of Object.values(document.paths ?? {})) {
    if (!pathItem || typeof pathItem !== 'object' || Array.isArray(pathItem)) {
      continue;
    }

    for (const [method, operation] of Object.entries(pathItem)) {
      if (!isOperationKey(method) || !operation || typeof operation !== 'object' || Array.isArray(operation)) {
        continue;
      }

      if (!isExplicitAnonymousSecurity(operation.security)) {
        operation.security = [cloneJson(dualTokenSecurity)];
      }

      for (const [statusCode, response] of Object.entries(operation.responses ?? {})) {
        if (!isErrorStatus(statusCode) || !response || typeof response !== 'object' || Array.isArray(response)) {
          continue;
        }
        const description = typeof response.description === 'string' && response.description.trim()
          ? response.description.trim()
          : 'Problem detail response';
        pathItem[method].responses[statusCode] = problemResponse(description);
      }
    }
  }

  const responses = document.components.responses && typeof document.components.responses === 'object'
    ? document.components.responses
    : {};
  for (const [name, response] of Object.entries(responses)) {
    if (!response || typeof response !== 'object' || Array.isArray(response)) {
      continue;
    }
    if (/Error$/.test(name)) {
      response.content = problemJsonContent();
    }
  }
  document.components.responses = responses;

  return document;
}
