from .http_client import HttpClient, SdkConfig
from .api.presence import PresenceApi
from .api.realtime import RealtimeApi
from .api.calls import CallsApi
from .api.social import SocialApi
from .api.chat import ChatApi
from .api.streams import StreamsApi
from .api.spaces import SpacesApi


class SdkworkImClient:
    """sdkwork-im-sdk SDK Client."""

    def __init__(self, config: SdkConfig):
        self._client = HttpClient(config)
        self.presence: PresenceApi
        self.realtime: RealtimeApi
        self.calls: CallsApi
        self.social: SocialApi
        self.chat: ChatApi
        self.streams: StreamsApi
        self.spaces: SpacesApi

        # Initialize API modules
        self.presence = PresenceApi(self._client)
        self.realtime = RealtimeApi(self._client)
        self.calls = CallsApi(self._client)
        self.social = SocialApi(self._client)
        self.chat = ChatApi(self._client)
        self.streams = StreamsApi(self._client)
        self.spaces = SpacesApi(self._client)
    def set_auth_token(self, token: str) -> 'SdkworkImClient':
        """Set auth token for authentication."""
        self._client.set_auth_token(token)
        return self

    def set_access_token(self, token: str) -> 'SdkworkImClient':
        """Set access token for authentication."""
        self._client.set_access_token(token)
        return self

    def set_header(self, key: str, value: str) -> 'SdkworkImClient':
        """Set custom header."""
        self._client.set_header(key, value)
        return self

    @property
    def http(self) -> HttpClient:
        """Get the underlying HTTP client."""
        return self._client


def create_client(config: SdkConfig) -> SdkworkImClient:
    """Create a new SDK client instance."""
    return SdkworkImClient(config)
