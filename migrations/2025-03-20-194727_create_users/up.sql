CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL UNIQUE
);

-- Add a sample user
INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com');