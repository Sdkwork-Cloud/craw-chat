import json as json_module

from sdkwork.common.core.types import SdkConfig as CommonSdkConfig
from sdkwork.common.http import BaseHttpClient

SdkConfig = CommonSdkConfig


class HttpClient(BaseHttpClient):
    """
    SDK HTTP client wrapper based on sdkwork-common.

    Auth headers:
    - auth_token -> Authorization: Bearer {auth_token}
    - access_token -> Access-Token: {access_token}
    """

    def _update_auth_headers(self) -> None:
        if self._session is None:
            return

        self._session.headers.pop('Authorization', None)
        self._session.headers.pop('Access-Token', None)
        self._session.headers.pop('X-API-Key', None)
        if self._auth_token:
            self._session.headers['Authorization'] = f'Bearer {self._auth_token}'
        if self._access_token:
            self._session.headers['Access-Token'] = self._access_token
    def set_auth_token(self, token: str) -> 'HttpClient':
        self._auth_token = token
        self._update_auth_headers()
        return self
    def set_access_token(self, token: str) -> 'HttpClient':
        self._access_token = token
        self._update_auth_headers()
        return self

    def set_header(self, key: str, value: str) -> 'HttpClient':
        self.headers[key] = value
        if self._session is not None:
            self._session.headers[key] = value
        return self

    def stream_json(self, path: str, method: str = 'POST', params=None, data=None, json=None, headers=None):
        response = self._get_session().request(
            method=method,
            url=f"{self.base_url}{path}",
            params=params,
            data=data,
            json=json,
            headers={'Accept': 'text/event-stream', **(headers or {})},
            timeout=self.timeout / 1000,
            stream=True,
        )
        response.raise_for_status()
        for line in response.iter_lines(decode_unicode=True):
            if not line or line.startswith(':'):
                continue
            if not line.startswith('data:'):
                continue
            data = line[5:].strip()
            if data == '[DONE]':
                break
            yield json_module.loads(data)
