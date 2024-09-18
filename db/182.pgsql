-- Time taken: 11:10, 11:11
SELECT DISTINCT
    email AS Email
FROM
    Person
GROUP BY
    email
HAVING
    count(email) > 1;

