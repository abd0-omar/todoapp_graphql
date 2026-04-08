--! create
INSERT INTO users (email, password_hash)
VALUES (:email, :password_hash)
RETURNING
    id,
    email,
    created_at;

--! load_by_email
SELECT
    id,
    email,
    password_hash,
    created_at
FROM
    users
WHERE
    email = :email;

--! load_by_id
SELECT
    id,
    email,
    created_at
FROM
    users
WHERE
    id = :id;
