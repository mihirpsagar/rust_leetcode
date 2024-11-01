-- Time taken: 12:11, 12:13 -> Acc
SELECT
    employee_id
FROM
    Employees
WHERE
    salary < 30000
    AND manager_id NOT IN ( SELECT DISTINCT
            employee_id
        FROM
            Employees)
ORDER BY
    employee_id;
