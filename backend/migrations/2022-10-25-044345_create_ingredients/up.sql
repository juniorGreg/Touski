-- Your SQL goes here
CREATE TABLE ingredients (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  UNIQUE(title)
)
