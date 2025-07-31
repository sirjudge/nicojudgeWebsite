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
