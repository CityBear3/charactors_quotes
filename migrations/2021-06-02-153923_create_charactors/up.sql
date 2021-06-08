-- Your SQL goes here
CREATE TABLE charactors (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  title VARCHAR(255) NOT NULL,
  quote VARCHAR(255) NOT NULL
)
