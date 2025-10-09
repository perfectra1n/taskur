# PostgreSQL Trigram Search Implementation

This document describes the robust, intuitive search functionality implemented using PostgreSQL trigram (pg_trgm) extension.

## Overview

The search system provides:
- **Fuzzy text matching** - Finds results even with typos and misspellings
- **Relevance scoring** - Results ranked by similarity
- **Multi-field search** - Searches across titles, descriptions, names, and content
- **Fast performance** - Uses GIN indexes for efficient querying
- **Unified search** - Search across all entity types (tasks, lists, tags, comments)

## Features

### 1. Trigram-Based Similarity Search

PostgreSQL trigram extension (`pg_trgm`) breaks text into three-character sequences to enable fuzzy matching:

```sql
-- Example: Searching for "documentation"
-- Will match: "documentation", "documentaton" (typo), "document", etc.
```

### 2. GIN Indexes

Fast text search enabled by GIN (Generalized Inverted Index) indexes on:
- Task titles and descriptions
- List names
- Tag names
- Comment content
- Team member names and emails

### 3. Relevance Scoring

Results are ranked using similarity scores (0.0 to 1.0+):
- **Title matches** - Weighted 3x for high relevance
- **Description matches** - Standard weight
- **Exact substring matches** - Boosted relevance
- **Similarity threshold** - 0.1 minimum (lenient fuzzy matching)

### 4. Two Search Modes

#### A. Task-Specific Search
**Endpoint**: `GET /api/tasks?search={query}`

Search within tasks with optional filters:
- Status (todo, inprogress, completed)
- Priority (low, medium, high, urgent)
- Tag
- List ID
- Limit (max results)

**Example**:
```bash
GET /api/tasks?search=implement&status=todo&priority=high&limit=20
```

**Response**:
```json
[
  {
    "id": "uuid",
    "title": "Implement authentication",
    "description": "Add JWT authentication...",
    "status": "todo",
    "priority": "high",
    "relevance": 0.85,
    ...
  }
]
```

#### B. Unified Search
**Endpoint**: `GET /api/search?q={query}&limit={max_results}`

Search across all entity types simultaneously.

**Example**:
```bash
GET /api/search?q=design&limit=50
```

**Response**:
```json
[
  {
    "entity_type": "task",
    "entity_id": "uuid-1",
    "title": "Design homepage",
    "description": "Create modern homepage design",
    "relevance": 0.92
  },
  {
    "entity_type": "list",
    "entity_id": "uuid-2",
    "title": "Design Tasks",
    "description": "",
    "relevance": 0.88
  },
  {
    "entity_type": "tag",
    "entity_id": "uuid-3",
    "title": "design-system",
    "description": "",
    "relevance": 0.75
  }
]
```

## Database Functions

### `search_tasks()`
Full-featured task search with relevance ranking and optional filters.

**Parameters**:
- `p_user_id` - User ID (required)
- `p_query` - Search query (required)
- `p_status` - Filter by status (optional)
- `p_priority` - Filter by priority (optional)
- `p_tag` - Filter by tag (optional)
- `p_list_id` - Filter by list (optional)
- `p_limit` - Max results (default: 50)

**Returns**: Task records with relevance scores

### `unified_search()`
Cross-entity search returning results from tasks, lists, tags, and comments.

**Parameters**:
- `p_user_id` - User ID (required)
- `p_query` - Search query (required)
- `p_limit` - Max results (default: 50)

**Returns**: Unified search results with entity type, ID, title, description, and relevance

## Search Quality Examples

### Exact Matches
```
Query: "authentication"
Matches: "authentication" (relevance: 1.0)
```

### Fuzzy Matches (Typos)
```
Query: "authentiction" (typo)
Matches: "authentication" (relevance: 0.7+)
```

### Partial Matches
```
Query: "auth"
Matches: "authentication", "authorization", "OAuth setup" (relevance: 0.5+)
```

### Word Order
```
Query: "user profile"
Matches: "User profile settings", "Profile for user", "User account profile"
```

## Performance Considerations

1. **GIN Indexes**: Pre-computed trigram indexes make searches very fast
2. **Similarity Threshold**: 0.1 threshold balances fuzzy matching vs. performance
3. **Result Limiting**: Default 50 results prevents overwhelming queries
4. **Index Maintenance**: Automatically updated on INSERT/UPDATE operations

## Migration

The search functionality is enabled via migration:
```
backend/migrations/20250109000003_add_trigram_search.sql
```

This migration:
1. Enables `pg_trgm` extension
2. Creates GIN indexes on searchable fields
3. Defines `search_tasks()` function
4. Defines `unified_search()` function

## Usage in Rust

### Task Search with Filters
```rust
// GET /api/tasks?search=implement&status=todo
let tasks = sqlx::query_as::<_, Task>(
    "SELECT * FROM search_tasks($1, $2, $3, $4, $5, $6, $7)"
)
.bind(user_id)
.bind("implement")
.bind(Some(TaskStatus::Todo))
.bind(None::<TaskPriority>)
.bind(None::<String>)
.bind(None::<Uuid>)
.bind(50)
.fetch_all(&pool)
.await?;
```

### Unified Search
```rust
// GET /api/search?q=design&limit=25
let results = sqlx::query_as::<_, UnifiedSearchResult>(
    "SELECT * FROM unified_search($1, $2, $3)"
)
.bind(user_id)
.bind("design")
.bind(25)
.fetch_all(&pool)
.await?;
```

## Testing Search

### Basic Search
```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8080/api/tasks?search=documentation"
```

### Search with Filters
```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8080/api/tasks?search=bug&status=todo&priority=urgent"
```

### Unified Search
```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8080/api/search?q=design&limit=20"
```

### Fuzzy Search Test
```bash
# Query with typo should still find "authentication"
curl -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8080/api/tasks?search=authentiction"
```

## Advanced Customization

### Adjusting Similarity Threshold

Edit the SQL functions to change the minimum similarity threshold:

```sql
-- Current: 0.1 (lenient)
WHERE similarity(t.title, p_query) > 0.1

-- Stricter matching: 0.3
WHERE similarity(t.title, p_query) > 0.3

-- Very strict: 0.5
WHERE similarity(t.title, p_query) > 0.5
```

### Custom Relevance Weights

Modify the `GREATEST()` calculation to adjust field weights:

```sql
-- Current weights
GREATEST(
    similarity(t.title, p_query) * 3.0,        -- Title 3x
    COALESCE(similarity(t.description, p_query), 0.0),  -- Description 1x
    ...
)

-- Custom weights (title 5x, description 2x)
GREATEST(
    similarity(t.title, p_query) * 5.0,
    COALESCE(similarity(t.description, p_query), 0.0) * 2.0,
    ...
)
```

## Troubleshooting

### Extension Not Found
```
ERROR: extension "pg_trgm" does not exist
```
**Solution**: Ensure PostgreSQL contrib package is installed:
```bash
# Ubuntu/Debian
sudo apt-get install postgresql-contrib

# macOS
brew install postgresql
```

### Slow Queries
1. Verify indexes exist: `\di` in psql
2. Analyze query plan: `EXPLAIN ANALYZE SELECT * FROM search_tasks(...)`
3. Consider increasing `similarity()` threshold to reduce matches

### No Results
1. Check similarity threshold (may be too strict)
2. Verify data exists for the user
3. Test with exact match first, then fuzzy

## Future Enhancements

Potential improvements:
1. **Weighted field search** - Let users specify which fields to prioritize
2. **Search suggestions** - "Did you mean..." based on similarity
3. **Search history** - Track and suggest previous searches
4. **Faceted search** - Show result counts by entity type
5. **Full-text search** - Combine trigram with `ts_vector` for even better text search
6. **Search filters UI** - Interactive frontend search with filter chips
