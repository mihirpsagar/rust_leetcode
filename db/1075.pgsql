-- Time taken: 20:52, 20:56 -> Wrong, 21:00 -> Acc
SELECT
    a.project_id,
    round(avg(b.experience_years), 2) AS average_years
FROM
    Project a
    INNER JOIN Employee b ON a.employee_id = b.employee_id
GROUP BY
    a.project_id
ORDER BY
    a.project_id;

