-- Time taken: 20:47, 20:55 -> Acc
SELECT
    product_name,
    unit
FROM (
    SELECT
        a.product_id,
        a.product_name,
        sum(b.unit) AS unit
    FROM
        Products a
        INNER JOIN Orders b ON a.product_id = b.product_id
    WHERE
        b.order_date BETWEEN '2020-02-01' AND '2020-02-29'
    GROUP BY
        a.product_id,
        a.product_name
    HAVING
        sum(b.unit) >= 100);

