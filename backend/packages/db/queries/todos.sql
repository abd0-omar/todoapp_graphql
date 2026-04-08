--! load_all
SELECT
    id,
    title,
    description,
    COALESCE(tags, ARRAY[]::text[]) AS tags,
    is_completed,
    created_at,
    updated_at
FROM
    todos;

--! load
SELECT
    id,
    title,
    description,
    COALESCE(tags, ARRAY[]::text[]) AS tags,
    is_completed,
    created_at,
    updated_at
FROM
    todos
WHERE
    id = :id;

--! create
INSERT INTO todos (
    title,
    description,
    is_completed,
    tags
) VALUES (
    :title,
    :description,
    :is_completed,
    :tags
)
RETURNING
    id,
    title,
    description,
    COALESCE(tags, ARRAY[]::text[]) AS tags,
    is_completed,
    created_at,
    updated_at;

--! update
UPDATE todos
SET
    title = :title,
    description = :description,
    is_completed = :is_completed,
    updated_at = CURRENT_TIMESTAMP
WHERE
    id = :id
RETURNING
    id,
    title,
    description,
    COALESCE(tags, ARRAY[]::text[]) AS tags,
    is_completed,
    created_at,
    updated_at;

--! update_including_tags
UPDATE todos
SET
    title = :title,
    description = :description,
    is_completed = :is_completed,
    tags = :tags,
    updated_at = CURRENT_TIMESTAMP
WHERE
    id = :id
RETURNING
    id,
    title,
    description,
    COALESCE(tags, ARRAY[]::text[]) AS tags,
    is_completed,
    created_at,
    updated_at;

--! update_tags
UPDATE todos
SET
    tags = :tags,
    updated_at = CURRENT_TIMESTAMP
WHERE
    id = :id
RETURNING
    id,
    title,
    description,
    COALESCE(tags, ARRAY[]::text[]) AS tags,
    is_completed,
    created_at,
    updated_at;

--! delete
DELETE FROM todos
WHERE
    id = :id
RETURNING
    id;
