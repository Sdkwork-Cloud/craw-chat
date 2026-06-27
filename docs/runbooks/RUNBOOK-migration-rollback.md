# RUNBOOK: Database Migration Rollback

## Trigger

- Failed database migration deployment
- Schema corruption after migration
- Rollback request from application team

## Prerequisites

- Admin access to Postgres (`SDKWORK_IM_DATABASE_ADMIN_URL`)
- Migration tooling (`pnpm db:postgres:migrate` or equivalent)
- Backup of database before migration

## Procedure

### 1. Assess migration status

```bash
# Check applied migrations
psql "$SDKWORK_IM_DATABASE_ADMIN_URL" -c "
  SELECT version, description, installed_on, success
  FROM schema_migrations
  ORDER BY version DESC LIMIT 10;
"
```

### 2. Stop affected services

```bash
# Stop session-gateway, projection-service, and audit-service
# to prevent writes during rollback
```

### 3. Restore from backup (if migration corrupted data)

```bash
# Restore from pre-migration snapshot
pg_restore -d "$SDKWORK_IM_DATABASE_ADMIN_URL" \
  --clean --if-exists \
  <backup-file>
```

### 4. Roll back specific migration (if tool supports)

```bash
# Apply down migration
pnpm db:postgres:rollback --to <previous-version>
```

### 5. Verify schema state

```bash
# Confirm schema matches previous version
psql "$SDKWORK_IM_DATABASE_ADMIN_URL" -c "
  SELECT version, description FROM schema_migrations
  ORDER BY version DESC LIMIT 5;
"
```

### 6. Restart services

```bash
# Restart services with rolling deployment
# Verify health checks pass
curl https://<gateway>/health
```

## Rollback

If rollback itself fails, restore from the pre-migration database backup and contact the database team.

## Escalation

Contact: SDKWork database team
