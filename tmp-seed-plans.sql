-- Token Bank recharge plans (order domain)
INSERT INTO commerce_token_bank_plan (id, tenant_id, organization_id, plan_code, display_name, plan_period, grant_amount, bonus_amount, price_amount, currency_code, renewal_policy, status, sort_weight, request_no, idempotency_key, created_at, updated_at)
VALUES
  ('tbp-100001-t100', '100001', NULL, 't100', '基础包', 'one_time', '100', '0', '10.00', 'CNY', 'none', 'active', 1, 'req-t100', 'idem-t100', now(), now()),
  ('tbp-100001-t500', '100001', NULL, 't500', '进阶包', 'one_time', '500', '50', '48.00', 'CNY', 'none', 'active', 2, 'req-t500', 'idem-t500', now(), now()),
  ('tbp-100001-t1000', '100001', NULL, 't1000', '专业包', 'one_time', '1000', '120', '95.00', 'CNY', 'none', 'active', 3, 'req-t1000', 'idem-t1000', now(), now()),
  ('tbp-100001-t5000', '100001', NULL, 't5000', '旗舰包', 'one_time', '5000', '800', '450.00', 'CNY', 'none', 'active', 4, 'req-t5000', 'idem-t5000', now(), now())
ON CONFLICT (id) DO NOTHING;

-- Membership plans (membership domain)
INSERT INTO membership_plan (id, uuid, tenant_id, organization_id, plan_no, plan_code, name, rank, description, status, version, created_at, updated_at)
VALUES
  ('mp-100001-vip', 'uuid-vip-plan', '100001', '0', 'VIP-MONTH', 'vip', 'VIP会员', 1, 'VIP membership plan', 'active', 0, now(), now())
ON CONFLICT (id) DO NOTHING;

-- Membership package groups
INSERT INTO membership_package_group (id, uuid, tenant_id, organization_id, external_id, group_no, name, description, billing_cycle, duration_days, display_channel, sort_weight, status, version, created_at, updated_at)
VALUES
  ('mpg-100001-vip', 'uuid-vip-group', '100001', '0', 1, 'VIP-GROUP', 'VIP会员套餐', 'VIP membership packages', 'monthly', 30, 'h5', 1, 'active', 0, now(), now())
ON CONFLICT (id) DO NOTHING;

-- Membership packages
INSERT INTO membership_package (id, uuid, tenant_id, organization_id, external_id, package_no, package_group_id, plan_id, name, description, price_amount, original_price_amount, currency_code, point_amount, duration_days, recurrence_cycle, sort_weight, recommended, status, tags, version, created_at, updated_at)
VALUES
  ('mpkg-100001-vip-month', 'uuid-vip-month', '100001', '0', 1, 'VIP-MONTH', 'mpg-100001-vip', 'mp-100001-vip', '连续包月', 'VIP monthly subscription', '19.00', '25.00', 'CNY', 0, 30, 'monthly', 1, 0, 'active', '["新用户"]'::jsonb, 0, now(), now()),
  ('mpkg-100001-vip-quarter', 'uuid-vip-quarter', '100001', '0', 2, 'VIP-QUARTER', 'mpg-100001-vip', 'mp-100001-vip', '连续包季', 'VIP quarterly subscription', '53.00', '75.00', 'CNY', 0, 90, 'quarterly', 2, 0, 'active', '["超值"]'::jsonb, 0, now(), now()),
  ('mpkg-100001-vip-year', 'uuid-vip-year', '100001', '0', 3, 'VIP-YEAR', 'mpg-100001-vip', 'mp-100001-vip', '连续包年', 'VIP yearly subscription', '188.00', '300.00', 'CNY', 0, 365, 'yearly', 3, 1, 'active', '["推荐","超值"]'::jsonb, 0, now(), now())
ON CONFLICT (id) DO NOTHING;
