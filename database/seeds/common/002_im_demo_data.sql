-- sdkwork:seed
-- id: 002_im_demo_data
-- engine: postgres
-- module: im
-- purpose: Provision IM demo data for the H5 chat and contacts surfaces
-- profile: standard
--
-- Ordering contract: this seed resolves IM principals from IAM users by login
-- name and FAILS CLOSED when the demo users are absent. Provision the IAM
-- accounts first with:
--
--   node scripts/dev/ensure-im-h5-demo-users.mjs
--
-- The seed is idempotent: every statement uses ON CONFLICT DO NOTHING and may
-- be re-run safely. Conversation state (conversations, messages, favorites) is
-- event-sourced by the conversation service write path and is therefore NOT
-- seeded here; provision it with:
--
--   node scripts/dev/ensure-im-h5-demo-conversations.mjs

DO $$
DECLARE
    missing text[] := ARRAY[]::text[];
    required_username text;
BEGIN
    FOREACH required_username IN ARRAY ARRAY['owner', 'guest', 'alice', 'bob', 'grace'] LOOP
        IF NOT EXISTS (
            SELECT 1 FROM iam_user
            WHERE username = required_username AND is_deleted = 0
        ) THEN
            missing := array_append(missing, required_username);
        END IF;
    END LOOP;    IF array_length(missing, 1) IS NOT NULL THEN
        RAISE EXCEPTION
            '002_im_demo_data requires IAM demo users %; run node scripts/dev/ensure-im-h5-demo-users.mjs first',
            array_to_string(missing, ', ');
    END IF;
END;
$$;

-- ===========================================================================
-- IM user profiles (read directly by the social contact and user profile APIs)
-- ===========================================================================

INSERT INTO im_user_profiles (
    tenant_id, organization_id, user_id,
    im_nickname, im_avatar_url, im_status_message,
    im_notification_prefs, im_mute_settings, im_privacy_settings,
    im_online_status, last_active_at, created_at, updated_at
)
SELECT
    '100001', '0', u.id,
    p.im_nickname, p.im_avatar_url, p.im_status_message,
    '{}'::jsonb, '{}'::jsonb, '{}'::jsonb,
    'offline', NOW() - INTERVAL '5 minutes', NOW(), NOW()
FROM iam_user u
JOIN (VALUES
    ('owner', '张明', 'https://cdn.sdkwork.com/demo/avatars/owner.png', '全力以赴做好每一件事'),
    ('guest', '访客', 'https://cdn.sdkwork.com/demo/avatars/guest.png', '你好，很高兴认识你'),
    ('alice', '李婷', 'https://cdn.sdkwork.com/demo/avatars/alice.png', '设计是一种态度'),
    ('bob', '王强', 'https://cdn.sdkwork.com/demo/avatars/bob.png', '代码改变世界'),
    ('grace', '陈静', 'https://cdn.sdkwork.com/demo/avatars/grace.png', '在路上')
) AS p(username, im_nickname, im_avatar_url, im_status_message)
    ON p.username = u.username
WHERE u.is_deleted = 0
ON CONFLICT (tenant_id, organization_id, user_id) DO NOTHING;

-- ===========================================================================
-- IM user settings (read directly by the user settings API)
-- ===========================================================================

INSERT INTO im_user_settings (tenant_id, organization_id, user_id, setting_key, setting_value, updated_at)
SELECT '100001', '0', u.id, s.setting_key, s.setting_value::jsonb, NOW()
FROM iam_user u
JOIN (VALUES
    ('owner', 'notify.message', '"true"'),
    ('owner', 'notify.friendRequest', '"true"'),
    ('owner', 'theme', '"light"'),
    ('owner', 'language', '"zh-CN"'),
    ('alice', 'notify.message', '"true"'),
    ('bob', 'notify.message', '"false"'),
    ('grace', 'theme', '"dark"')
) AS s(username, setting_key, setting_value)
    ON s.username = u.username
WHERE u.is_deleted = 0
ON CONFLICT (tenant_id, organization_id, user_id, setting_key) DO NOTHING;

-- ===========================================================================
-- Contact tags (read directly by the social contact tags API)
-- ===========================================================================

INSERT INTO im_contact_tags (
    tenant_id, organization_id, owner_user_id, tag_id, name, color, count, bg, border, created_at, updated_at
)
SELECT
    '100001', '0', u.id, t.tag_id, t.name, t.color, t.count, t.bg, t.border, NOW(), NOW()
FROM iam_user u
JOIN (VALUES
    ('owner', 1, '同事', '#1677ff', 8, '#e6f4ff', '#91caff'),
    ('owner', 2, '家人', '#52c41a', 3, '#f6ffed', '#b7eb8f'),
    ('owner', 3, '客户', '#fa8c16', 5, '#fff7e6', '#ffd591'),
    ('owner', 4, '重要', '#f5222d', 2, '#fff1f0', '#ffa39e')
) AS t(username, tag_id, name, color, count, bg, border)
    ON t.username = u.username
WHERE u.is_deleted = 0
ON CONFLICT (tenant_id, organization_id, owner_user_id, tag_id) DO NOTHING;

-- ===========================================================================
-- Friendships (read directly by the social contacts API)
-- user_low_id / user_high_id follow the normalized lexical pair ordering
-- ===========================================================================

INSERT INTO im_friendships (
    tenant_id, organization_id, friendship_id, user_low_id, user_high_id,
    initiator_user_id, status, established_at, updated_at
)
SELECT
    '100001', '0', f.friendship_id,
    LEAST(owner.id, peer.id), GREATEST(owner.id, peer.id),
    owner.id, 'active',
    NOW() - (f.established_days_ago || ' days')::interval,
    NOW() - (f.established_days_ago || ' days')::interval
FROM iam_user owner
JOIN (VALUES
    ('owner', 'alice', 700000000000000001, 30),
    ('owner', 'bob', 700000000000000002, 14),
    ('owner', 'guest', 700000000000000003, 7),
    ('alice', 'bob', 700000000000000004, 10)
) AS f(owner_username, peer_username, friendship_id, established_days_ago)
    ON f.owner_username = owner.username
JOIN iam_user peer ON peer.username = f.peer_username
WHERE owner.is_deleted = 0 AND peer.is_deleted = 0
ON CONFLICT (tenant_id, organization_id, friendship_id) DO NOTHING;

-- ===========================================================================
-- Friend requests (read directly by the social friend request APIs)
-- guest -> owner pending: the owner sees one incoming pending request
-- owner -> grace pending: the owner sees one outgoing pending request
-- alice -> owner accepted: history in the new-friends list
-- ===========================================================================

INSERT INTO im_friend_requests (
    tenant_id, organization_id, request_id, requester_user_id, target_user_id,
    request_message, status, expired_at, created_at, updated_at
)
SELECT
    '100001', '0', r.request_id, requester.id, target.id,
    r.request_message, r.status, NULL,
    NOW() - (r.created_days_ago || ' days')::interval,
    NOW() - (r.created_days_ago || ' days')::interval
FROM iam_user requester
JOIN (VALUES
    ('guest', 'owner', 700000000000000101, '你好，我是访客，想加你为好友', 'pending', 2),
    ('owner', 'grace', 700000000000000102, '陈静你好，我是张明', 'pending', 1),
    ('alice', 'owner', 700000000000000103, '上次会议认识，加个好友吧', 'accepted', 12)
) AS r(requester_username, target_username, request_id, request_message, status, created_days_ago)
    ON r.requester_username = requester.username
JOIN iam_user target ON target.username = r.target_username
WHERE requester.is_deleted = 0 AND target.is_deleted = 0
ON CONFLICT (tenant_id, organization_id, request_id) DO NOTHING;
