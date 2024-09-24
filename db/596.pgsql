-- Time taken: 00:15, 00:16
SELECT
    class
FROM
    Courses
GROUP BY
    class
HAVING
    count(student) > 4;

