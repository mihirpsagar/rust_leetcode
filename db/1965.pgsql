-- Time taken: 20:50, 20:53 -> Acc, 20:56 -> Optimized
SELECT
    coalesce(a.employee_id, b.employee_id) AS employee_id
FROM
    Employees a
    FULL JOIN Salaries b ON a.employee_id = b.employee_id
WHERE
    a.employee_id IS NULL
    OR b.employee_id IS NULL
ORDER BY
    employee_id ASC;

-- SELECT DISTINCT
--     employee_id
-- FROM (
--     SELECT
--         employee_id
--     FROM
--         Employees
--     UNION
--     SELECT
--         employee_id
--     FROM
--         Salaries)
-- WHERE
--     employee_id NOT IN (
--         SELECT
--             a.employee_id
--         FROM
--             Employees a
--             INNER JOIN Salaries b ON a.employee_id = b.employee_id)
-- ORDER BY
--     employee_id ASC;
