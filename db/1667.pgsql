-- Time taken: 17:49, 17:53 -> Acc
SELECT
    user_id,
    UPPER(SUBSTRING(name, 1, 1)) || LOWER(SUBSTRING(name, 2)) AS name
FROM
    Users
ORDER BY
    user_id ASC;

