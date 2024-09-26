-- Time taken: 23:19, 23:20 -> Acc
SELECT
    *
FROM
    Cinema
WHERE
    id % 2 = 1
    AND description != 'boring'
ORDER BY
    rating DESC;

