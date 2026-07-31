import { configureCommunityAuthSessionPort } from '@sdkwork/community-mobile-react-community';
import { AuthService } from '@sdkwork/im-h5-user';

configureCommunityAuthSessionPort(AuthService);

/** Compatibility adapter. Canonical Community mobile UI lives in sdkwork-community. */
export * from '@sdkwork/community-mobile-react-community';
