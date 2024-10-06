-- Time taken: 21:10, 21:12, 21:15 -> Wrong, 21:16 -> Acc
SELECT DISTINCT
    a.product_id,
    a.product_name
FROM
    Product a
    INNER JOIN Sales b ON a.product_id = b.product_id
WHERE
    b.sale_date >= '2019-01-01'
    AND b.sale_date <= '2019-03-31'
    AND a.product_id NOT IN ( SELECT DISTINCT
            product_id
        FROM
            Sales
        WHERE
            sale_date < '2019-01-01'
            OR sale_date > '2019-03-31');

-- WITH sales_range AS (
--     SELECT
--         product_id,
--         min(sale_date) AS min_date,
--         max(sale_date) AS max_date
--     FROM
--         Sales
--     GROUP BY
--         product_id
-- )
-- SELECT
--     a.product_id,
--     b.product_name
-- FROM
--     sales_range AS a
--     LEFT JOIN Product b ON a.product_id = b.product_id
-- WHERE
--     a.min_date >= '2019-01-01'
--     AND max_date <= '2019-03-31';
