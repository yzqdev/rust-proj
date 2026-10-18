-- 任务表：对外用 public_id(UUID) 暴露，自增 id 仅内部使用。
CREATE TABLE IF NOT EXISTS tasks (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    public_id   TEXT    NOT NULL UNIQUE,
    title       TEXT    NOT NULL,
    description TEXT,
    status      TEXT    NOT NULL DEFAULT 'todo'
                CHECK (status IN ('todo', 'in_progress', 'done', 'cancelled')),
    priority    INTEGER NOT NULL DEFAULT 2
                CHECK (priority BETWEEN 0 AND 4),
    due_at      TEXT,
    created_at  TEXT    NOT NULL,
    updated_at  TEXT    NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_tasks_status     ON tasks (status);
CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks (created_at DESC);
