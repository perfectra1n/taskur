-- Enable the PostgreSQL trigram extension for fuzzy text search
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Create GIN indexes using trigram operators for fast similarity searches
-- These indexes enable efficient fuzzy matching and typo tolerance

-- Tasks table: Index for title and description search
CREATE INDEX idx_tasks_title_trgm ON tasks USING GIN (title gin_trgm_ops);
CREATE INDEX idx_tasks_description_trgm ON tasks USING GIN (description gin_trgm_ops);

-- Lists table: Index for name search
CREATE INDEX idx_lists_name_trgm ON lists USING GIN (name gin_trgm_ops);

-- Tags table: Index for tag name search
CREATE INDEX idx_tags_name_trgm ON tags USING GIN (name gin_trgm_ops);

-- Comments table: Index for comment content search
CREATE INDEX idx_comments_content_trgm ON comments USING GIN (content gin_trgm_ops);

-- Team members table: Index for name and email search
CREATE INDEX idx_team_members_name_trgm ON team_members USING GIN (name gin_trgm_ops);
CREATE INDEX idx_team_members_email_trgm ON team_members USING GIN (email gin_trgm_ops);

-- Create a function to search tasks with relevance scoring
CREATE OR REPLACE FUNCTION search_tasks(
    p_user_id UUID,
    p_query TEXT,
    p_status task_status DEFAULT NULL,
    p_priority task_priority DEFAULT NULL,
    p_tag TEXT DEFAULT NULL,
    p_list_id UUID DEFAULT NULL,
    p_limit INTEGER DEFAULT 50
)
RETURNS TABLE (
    id UUID,
    user_id UUID,
    title VARCHAR(500),
    description TEXT,
    status task_status,
    priority task_priority,
    due_date TIMESTAMPTZ,
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,
    hero_image_id UUID,
    assigned_to UUID[],
    reminders JSONB,
    tags TEXT[],
    position INTEGER,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    relevance FLOAT
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        t.id,
        t.user_id,
        t.title,
        t.description,
        t.status,
        t.priority,
        t.due_date,
        t.start_date,
        t.end_date,
        t.hero_image_id,
        t.assigned_to,
        t.reminders,
        t.tags,
        t.position,
        t.created_at,
        t.updated_at,
        -- Calculate relevance score using trigram similarity
        -- Title matches are weighted 3x more than description matches
        GREATEST(
            similarity(t.title, p_query) * 3.0,
            COALESCE(similarity(t.description, p_query), 0.0),
            -- Boost exact substring matches
            CASE WHEN t.title ILIKE '%' || p_query || '%' THEN 0.9 ELSE 0.0 END,
            CASE WHEN t.description ILIKE '%' || p_query || '%' THEN 0.6 ELSE 0.0 END
        ) AS relevance
    FROM tasks t
    WHERE t.user_id = p_user_id
        -- Apply basic similarity threshold (0.1 is quite lenient for fuzzy matching)
        AND (
            similarity(t.title, p_query) > 0.1
            OR COALESCE(similarity(t.description, p_query), 0) > 0.1
            OR t.title ILIKE '%' || p_query || '%'
            OR t.description ILIKE '%' || p_query || '%'
        )
        -- Apply optional filters
        AND (p_status IS NULL OR t.status = p_status)
        AND (p_priority IS NULL OR t.priority = p_priority)
        AND (p_tag IS NULL OR p_tag = ANY(t.tags))
        AND (p_list_id IS NULL OR EXISTS (
            SELECT 1 FROM task_lists tl
            WHERE tl.task_id = t.id AND tl.list_id = p_list_id
        ))
    ORDER BY relevance DESC, t.created_at DESC
    LIMIT p_limit;
END;
$$ LANGUAGE plpgsql;

-- Create a unified search function across all entities
CREATE OR REPLACE FUNCTION unified_search(
    p_user_id UUID,
    p_query TEXT,
    p_limit INTEGER DEFAULT 50
)
RETURNS TABLE (
    entity_type TEXT,
    entity_id UUID,
    title TEXT,
    description TEXT,
    relevance FLOAT
) AS $$
BEGIN
    RETURN QUERY
    -- Search tasks
    SELECT
        'task'::TEXT AS entity_type,
        t.id AS entity_id,
        t.title AS title,
        COALESCE(t.description, '')::TEXT AS description,
        GREATEST(
            similarity(t.title, p_query) * 3.0,
            COALESCE(similarity(t.description, p_query), 0.0)
        ) AS relevance
    FROM tasks t
    WHERE t.user_id = p_user_id
        AND (
            similarity(t.title, p_query) > 0.1
            OR COALESCE(similarity(t.description, p_query), 0) > 0.1
            OR t.title ILIKE '%' || p_query || '%'
            OR t.description ILIKE '%' || p_query || '%'
        )

    UNION ALL

    -- Search lists
    SELECT
        'list'::TEXT AS entity_type,
        l.id AS entity_id,
        l.name AS title,
        ''::TEXT AS description,
        similarity(l.name, p_query) * 2.0 AS relevance
    FROM lists l
    WHERE l.user_id = p_user_id
        AND (
            similarity(l.name, p_query) > 0.1
            OR l.name ILIKE '%' || p_query || '%'
        )

    UNION ALL

    -- Search tags
    SELECT
        'tag'::TEXT AS entity_type,
        tg.id AS entity_id,
        tg.name AS title,
        ''::TEXT AS description,
        similarity(tg.name, p_query) * 2.0 AS relevance
    FROM tags tg
    WHERE tg.user_id = p_user_id
        AND (
            similarity(tg.name, p_query) > 0.1
            OR tg.name ILIKE '%' || p_query || '%'
        )

    UNION ALL

    -- Search comments
    SELECT
        'comment'::TEXT AS entity_type,
        c.id AS entity_id,
        'Comment'::TEXT AS title,
        c.content::TEXT AS description,
        similarity(c.content, p_query) AS relevance
    FROM comments c
    WHERE c.user_id = p_user_id
        AND (
            similarity(c.content, p_query) > 0.1
            OR c.content ILIKE '%' || p_query || '%'
        )

    ORDER BY relevance DESC
    LIMIT p_limit;
END;
$$ LANGUAGE plpgsql;
