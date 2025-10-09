-- Add new fields to tasks table
ALTER TABLE tasks
    ADD COLUMN start_date TIMESTAMPTZ,
    ADD COLUMN end_date TIMESTAMPTZ,
    ADD COLUMN hero_image_id UUID REFERENCES attachments(id) ON DELETE SET NULL,
    ADD COLUMN assigned_to UUID[] NOT NULL DEFAULT '{}',
    ADD COLUMN reminders JSONB NOT NULL DEFAULT '[]';

-- Create indexes for better query performance
CREATE INDEX idx_tasks_start_date ON tasks(user_id, start_date);
CREATE INDEX idx_tasks_end_date ON tasks(user_id, end_date);
CREATE INDEX idx_tasks_hero_image ON tasks(hero_image_id);
CREATE INDEX idx_tasks_assigned_to ON tasks USING GIN(assigned_to);
CREATE INDEX idx_tasks_reminders ON tasks USING GIN(reminders);

-- Create team_members table for team collaboration
CREATE TABLE team_members (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    avatar_url VARCHAR(500),
    role VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_team_members_user_id ON team_members(user_id);
CREATE INDEX idx_team_members_email ON team_members(user_id, email);

-- Create task_assignments table for tracking detailed assignment information (optional - for future use)
-- For now, we use the assigned_to array directly in the tasks table
-- Uncomment this if you want detailed assignment tracking:
-- CREATE TABLE task_assignments (
--     id UUID PRIMARY KEY,
--     task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
--     assignee_id UUID NOT NULL REFERENCES team_members(id) ON DELETE CASCADE,
--     assigned_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
--     assigned_at TIMESTAMPTZ NOT NULL,
--     CONSTRAINT task_assignments_pkey UNIQUE (task_id, assignee_id)
-- );
--
-- CREATE INDEX idx_task_assignments_task_id ON task_assignments(task_id);
-- CREATE INDEX idx_task_assignments_assignee_id ON task_assignments(assignee_id);

-- Add constraint to ensure end_date is after start_date when both are set
ALTER TABLE tasks ADD CONSTRAINT check_date_order
    CHECK (start_date IS NULL OR end_date IS NULL OR start_date <= end_date);
