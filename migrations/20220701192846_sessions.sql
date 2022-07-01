-- Add migration script here
CREATE TABLE IF NOT EXISTS sessions (
  `id` VARCHAR(128) NOT NULL,
  `expires` DATETIME NOT NULL,
  `session` TEXT NOT NULL,
  PRIMARY KEY (`id`)
);
