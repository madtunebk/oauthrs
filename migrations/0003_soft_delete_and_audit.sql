-- Soft delete + audit log for users

ALTER TABLE users ADD COLUMN IF NOT EXISTS deleted_at  TIMESTAMPTZ DEFAULT NULL;
ALTER TABLE users ADD COLUMN IF NOT EXISTS disabled_at TIMESTAMPTZ DEFAULT NULL;
ALTER TABLE users ADD COLUMN IF NOT EXISTS deleted_by  UUID        DEFAULT NULL;  -- admin user_id

CREATE TABLE IF NOT EXISTS user_audit_log (
    id          BIGSERIAL   PRIMARY KEY,
    user_id     UUID        NOT NULL,
    action      TEXT        NOT NULL,  -- 'deleted', 'disabled', 'enabled', 'created', 'google_linked'
    performed_by UUID       DEFAULT NULL,  -- NULL = system
    note        TEXT        DEFAULT '',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_audit_user    ON user_audit_log (user_id);
CREATE INDEX IF NOT EXISTS idx_audit_created ON user_audit_log (created_at DESC);
