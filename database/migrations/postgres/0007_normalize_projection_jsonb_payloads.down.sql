BEGIN;

DO $$
BEGIN
    RAISE EXCEPTION
        'rollback refused: normalized projection JSONB values are the canonical storage form';
END;
$$;

COMMIT;
