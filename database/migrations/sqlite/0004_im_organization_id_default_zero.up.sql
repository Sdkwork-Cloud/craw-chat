-- Align IM organization scope sentinel with SUBJECT_ID_SPEC tenant-level default `0`.
-- Historical rows may still carry the legacy TEXT sentinel `default`.

UPDATE im_commit_journal
SET organization_id = '0'
WHERE organization_id = 'default';
