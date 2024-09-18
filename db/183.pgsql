-- Time taken: 11:13, 11:15
SELECT
    c.name AS Customers
FROM
    Customers c
    LEFT JOIN Orders o ON c.id = o.customerId
WHERE
    o.id IS NULL;

