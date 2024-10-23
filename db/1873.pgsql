-- Time taken: 18:28, 18:30 -> Wrong, 18:31 -> Acc
SELECT
    employee_id,
    CASE WHEN employee_id % 2 = 1
        AND SUBSTRING(name, 1, 1) != 'M' THEN
        salary
    ELSE
        0
    END AS bonus
FROM
    Employees
ORDER BY
    employee_id ASC;

