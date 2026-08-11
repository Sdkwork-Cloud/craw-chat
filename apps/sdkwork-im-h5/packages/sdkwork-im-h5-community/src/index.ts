import { configureCommunityAuthSessionPort } from '@sdkwork/community-mobile-react-community';
import { AuthService } from '@sdkwork/im-h5-user';

// Auth session port for the community payment sheet: serves the REAL IAM
// current user (resolved by AuthGate from `iam.users.current.retrieve()` and
// held in the app store). No demo user or fabricated session is ever used.
configureCommunityAuthSessionPort(AuthService);

/** Compatibility adapter. Canonical Community mobile UI lives in sdkwork-community. */
export * from '@sdkwork/community-mobile-react-community';
