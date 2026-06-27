const HTTP_METHODS = new Set(['delete', 'get', 'patch', 'post', 'put', 'options', 'head']);

export function applyWebFrameworkOpenApiExtensions(document, apiSurface) {
  let changed = 0;
  const paths = document.paths ?? {};
  for (const pathItem of Object.values(paths)) {
    if (!pathItem || typeof pathItem !== 'object') {
      continue;
    }
    for (const [method, operation] of Object.entries(pathItem)) {
      if (!HTTP_METHODS.has(method)) {
        continue;
      }
      if (!operation || typeof operation !== 'object') {
        continue;
      }
      if (operation['x-sdkwork-request-context'] !== 'WebRequestContext') {
        operation['x-sdkwork-request-context'] = 'WebRequestContext';
        changed += 1;
      }
      if (operation['x-sdkwork-api-surface'] !== apiSurface) {
        operation['x-sdkwork-api-surface'] = apiSurface;
        changed += 1;
      }
    }
  }
  return changed;
}

export const IM_OPENAPI_AUTHORITY_TARGETS = [
  {
    relativePath: 'sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml',
    apisAuthorityPath: 'apis/open-api/im/sdkwork-im-im.openapi.yaml',
    apiSurface: 'open-api',
  },
  {
    relativePath: 'sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml',
    apisAuthorityPath: 'apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml',
    apiSurface: 'backend-api',
  },
  {
    relativePath: 'sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.openapi.yaml',
    apisAuthorityPath: 'apis/app-api/communication/sdkwork-im-app-api.openapi.yaml',
    apiSurface: 'app-api',
  },
];
