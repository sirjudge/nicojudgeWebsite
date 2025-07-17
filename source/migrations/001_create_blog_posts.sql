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
)
