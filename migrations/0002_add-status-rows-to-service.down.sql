- Remove the status column
ALTER TABLE service DROP COLUMN status;

-- Remove the run_command column
ALTER TABLE service_config DROP COLUMN run_command;