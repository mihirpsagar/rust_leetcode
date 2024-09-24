-- Time taken: 23:12, 23:13
SELECT
    name
FROM
    Customer
WHERE
    referee_id != 2
    OR referee_id IS NULL;

