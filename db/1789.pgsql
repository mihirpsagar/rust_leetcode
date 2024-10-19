-- Time taken: 13:35, 13:37, 13:43 -> Acc
SELECT
    employee_id,
    department_id
FROM
    Employee
WHERE
    primary_flag = 'Y'
UNION
SELECT
    employee_id,
    department_id
FROM
    Employee
WHERE
    employee_id IN (
        SELECT
            employee_id
        FROM
            Employee
        GROUP BY
            employee_id
        HAVING
            count(employee_id) = 1);

-- SELECT DISTINCT ON (employee_id)
--     employee_id,
--     department_id
-- FROM
--     Employee
-- ORDER BY
--     primary_flag DESC;
