CREATE TABLE IF NOT EXISTS blog_posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS web_flags(
    maintenance_mode BIT NOT NULL,
    updated_Date DATETIME NOT NULL
);

create TABLE IF NOT EXISTS roles (
    role_id SMALLINT NOT NULL PRIMARY KEY,
    role_name CHARACTER(20) NOT NULL
);

INSERT INTO roles (role_id, role_name) VALUES
(1, "admin"), (2, "user"), (3, "guest");

create TABLE IF NOT EXISTS accounts(
    account_id INTEGER PRIMARY KEY AUTOINCREMENT,
    username character(50) NOT NULL,
    password_hash TEXT NOT NULL,
    role_id smallint NOT NULL,
    FOREIGN KEY(role_id) REFERENCES roles(role_id)
);

-- Sessions table for managing user sessions
CREATE TABLE IF NOT EXISTS sessions (
    session_id TEXT PRIMARY KEY,
    account_id INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    last_accessed DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ip_address TEXT,
    user_agent TEXT,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    FOREIGN KEY(account_id) REFERENCES accounts(account_id) ON DELETE CASCADE
);

-- Index for faster session lookups
CREATE INDEX IF NOT EXISTS idx_sessions_account_id ON sessions(account_id);
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at);
CREATE INDEX IF NOT EXISTS idx_sessions_active ON sessions(is_active, expires_at);
