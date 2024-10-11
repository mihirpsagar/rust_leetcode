-- Time taken: 01:30, 01:32 -> Acc
SELECT
    b.unique_id,
    a.name
FROM
    Employees a
    LEFT JOIN EmployeeUNI b ON a.id = b.id;

