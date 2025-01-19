-- Add up migration script here
ALTER TABLE service_config ADD COLUMN run_command TEXT;
ALTER TABLE service ADD COLUMN status TEXT CHECK(status IN ('Starting', 'Running', 'Stopped', 'Failed')) NOT NULL DEFAULT 'Stopped';