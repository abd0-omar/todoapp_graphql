--! load_all
SELECT
    id,
    title,
    description,
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
    is_completed
) VALUES (
    :title,
    :description,
    :is_completed
)
RETURNING
    id,
    title,
    description,
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
    is_completed,
    created_at,
    updated_at;

--! delete
DELETE FROM todos
WHERE
    id = :id
RETURNING
    id;
