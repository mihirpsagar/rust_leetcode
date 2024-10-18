-- Time taken: 23:36, 23:42, 23:46 -> Wrong, 23:47 -> Acc
SELECT
    a.employee_id,
    a.name,
    b.reports_count,
    b.average_age
FROM
    Employees a
    INNER JOIN (
        SELECT
            reports_to,
            count(reports_to) AS reports_count,
            round(avg(age)) AS average_age
        FROM
            Employees
        WHERE
            reports_to IS NOT NULL
        GROUP BY
            reports_to) b ON a.employee_id = b.reports_to
ORDER BY
    a.employee_id ASC;

