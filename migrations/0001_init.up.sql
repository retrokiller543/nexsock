-- Add up migration script here
-- Services table to store main service information
CREATE TABLE services (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    config_id INTEGER NOT NULL,
    name TEXT NOT NULL UNIQUE,
    repo_url TEXT NOT NULL,
    port INTEGER NOT NULL,
    repo_path TEXT NOT NULL,
    FOREIGN KEY (config_id) REFERENCES service_config(id)
);

-- Dependant services relationship table
CREATE TABLE service_dependencies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    service_id INTEGER NOT NULL,
    dependent_service_id INTEGER NOT NULL CHECK(dependent_service_id != service_id),
    tunnel_enabled BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (service_id) REFERENCES services(id),
    FOREIGN KEY (dependent_service_id) REFERENCES services(id)
);
CREATE UNIQUE INDEX "service_dep_idx" ON "service_dependencies" ("service_id","dependent_service_id");

-- Service config updates table for detailed dependency configurations
CREATE TABLE service_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    filename TEXT NOT NULL,
    format TEXT CHECK(format IN ('Env', 'Properties')) DEFAULT 'Env'
);