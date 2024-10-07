-- Time taken: 00:50, 00:51 -> Acc
SELECT DISTINCT
    author_id AS id
FROM
    Views
WHERE
    author_id = viewer_id
ORDER BY
    author_id ASC;

