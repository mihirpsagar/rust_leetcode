-- Time taken: 12:08, 12:13
DELETE FROM person p1 USING person p2
WHERE p1.email = p2.email
    AND p1.id > p2.id;

-- DELETE FROM person
-- WHERE id NOT IN (
--         SELECT
--             min(id)
--         FROM
--             person
--         GROUP BY
--             email);
