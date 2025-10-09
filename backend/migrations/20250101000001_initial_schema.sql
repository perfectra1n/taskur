-- Create custom types
CREATE TYPE task_status AS ENUM ('todo', 'inprogress', 'completed');
CREATE TYPE task_priority AS ENUM ('low', 'medium', 'high', 'urgent');

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_users_email ON users(email);

-- Lists table
CREATE TABLE lists (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    color VARCHAR(50),
    icon VARCHAR(50),
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_lists_user_id ON lists(user_id);
CREATE INDEX idx_lists_position ON lists(user_id, position);

-- Tags table
CREATE TABLE tags (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(50) NOT NULL,
    color VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_tags_user_id ON tags(user_id);
CREATE UNIQUE INDEX idx_tags_user_name ON tags(user_id, name);

-- Tasks table
CREATE TABLE tasks (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    status task_status NOT NULL DEFAULT 'todo',
    priority task_priority NOT NULL DEFAULT 'medium',
    due_date TIMESTAMPTZ,
    tags TEXT[] NOT NULL DEFAULT '{}',
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_tasks_user_id ON tasks(user_id);
CREATE INDEX idx_tasks_status ON tasks(user_id, status);
CREATE INDEX idx_tasks_priority ON tasks(user_id, priority);
CREATE INDEX idx_tasks_due_date ON tasks(user_id, due_date);
CREATE INDEX idx_tasks_position ON tasks(user_id, position);
CREATE INDEX idx_tasks_tags ON tasks USING GIN(tags);

-- Task-List junction table
CREATE TABLE task_lists (
    task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    list_id UUID NOT NULL REFERENCES lists(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, list_id)
);

CREATE INDEX idx_task_lists_task_id ON task_lists(task_id);
CREATE INDEX idx_task_lists_list_id ON task_lists(list_id);

-- Comments table
CREATE TABLE comments (
    id UUID PRIMARY KEY,
    task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_comments_task_id ON comments(task_id);
CREATE INDEX idx_comments_user_id ON comments(user_id);

-- Attachments table
CREATE TABLE attachments (
    id UUID PRIMARY KEY,
    task_id UUID REFERENCES tasks(id) ON DELETE CASCADE,
    comment_id UUID REFERENCES comments(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    file_size BIGINT NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT attachment_parent_check CHECK (
        (task_id IS NOT NULL AND comment_id IS NULL) OR
        (task_id IS NULL AND comment_id IS NOT NULL)
    )
);

CREATE INDEX idx_attachments_task_id ON attachments(task_id);
CREATE INDEX idx_attachments_comment_id ON attachments(comment_id);
CREATE INDEX idx_attachments_user_id ON attachments(user_id);
