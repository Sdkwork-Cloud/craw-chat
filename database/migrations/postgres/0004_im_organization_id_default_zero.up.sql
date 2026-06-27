-- Align IM organization scope sentinel with SUBJECT_ID_SPEC tenant-level default `0`.
-- Historical rows and column defaults used the legacy TEXT sentinel `default`.

UPDATE im_commit_journal
SET organization_id = '0'
WHERE organization_id = 'default';

ALTER TABLE im_commit_journal
    ALTER COLUMN organization_id SET DEFAULT '0';
