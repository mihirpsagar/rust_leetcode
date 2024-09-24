-- Time taken: 23:07, 23:09
SELECT
    a.name,
    b.bonus
FROM
    Employee a
    LEFT OUTER JOIN bonus b ON a.empId = b.empId
WHERE
    b.bonus < 1000
    OR b.bonus IS NULL;

