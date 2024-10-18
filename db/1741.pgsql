-- Time taken: 23:58, 23:59 -> Acc
SELECT
    emp_id,
    event_day,
    sum(out_time - in_time) AS total_time
FROM
    Employees
GROUP BY
    emp_id,
    event_day;

