-- Add down migration script here
-- Drop the index if it exists
DROP INDEX IF EXISTS service_dep_idx;

-- Drop tables if they exist
DROP TABLE IF EXISTS service_dependency;
DROP TABLE IF EXISTS service;
DROP TABLE IF EXISTS service_config;
