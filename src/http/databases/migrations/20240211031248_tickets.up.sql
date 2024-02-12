-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS tickets (
  id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
  subject VARCHAR(255) NOT NULL,
  description TEXT,
  status INT NOT NULL DEFAULT 1,
  priority INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
