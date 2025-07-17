-- Create blog_posts table
-- This migration creates the blog_posts table with the same structure as the original Diesel migration
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
    role_id SMALLINT NOT NULL,
    role_name CHARACTER(20) NOT NULL
);

INSERT INTO ROWS (role_id, role_name) VALUES
(1, "admin"), (2, "user"), (3, "guest");

create TABLE IF NOT EXISTS accounts(
    account_id SMALLINT NOT NULL,
    username character(50) NOT NULL
    password_hash TEXT NOT NULL
    FOREIGN KEY(role_id) REFERENCES roles(role_id)
);
