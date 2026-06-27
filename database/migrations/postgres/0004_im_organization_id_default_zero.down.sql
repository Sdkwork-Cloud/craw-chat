ALTER TABLE im_commit_journal
    ALTER COLUMN organization_id SET DEFAULT 'default';

UPDATE im_commit_journal
SET organization_id = 'default'
WHERE organization_id = '0';
