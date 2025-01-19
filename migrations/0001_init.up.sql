-- Add up migration script here
-- Services table to store main service information
CREATE TABLE service (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    config_id INTEGER,
    name TEXT NOT NULL UNIQUE,
    repo_url TEXT NOT NULL,
    port INTEGER NOT NULL,
    repo_path TEXT NOT NULL,
    FOREIGN KEY (config_id) REFERENCES service_config(id)
);

-- Dependant services relationship table
CREATE TABLE service_dependency (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    service_id INTEGER NOT NULL,
    dependent_service_id INTEGER NOT NULL CHECK(dependent_service_id != service_id),
    tunnel_enabled BOOLEAN DEFAULT FALSE NOT NULL,
    FOREIGN KEY (service_id) REFERENCES service(id),
    FOREIGN KEY (dependent_service_id) REFERENCES service(id)
);
CREATE UNIQUE INDEX "service_dep_idx" ON "service_dependency" ("service_id","dependent_service_id");

-- Service config updates table for detailed dependency configurations
CREATE TABLE service_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    filename TEXT NOT NULL,
    format TEXT CHECK(format IN ('Env', 'Properties')) DEFAULT 'Env'
);