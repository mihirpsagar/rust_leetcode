-- Time taken: 18:28, 18:42 -> Wrong, 18:43 -> Acc, 18:46 -> Optimized
SELECT
    a.product_id,
    coalesce(round((sum(a.price * b.units)::float / sum(b.units)::float)::numeric, 2), 0) AS average_price
FROM
    Prices a
    LEFT JOIN UnitsSold b ON a.product_id = b.product_id
        AND b.purchase_date BETWEEN a.start_date AND a.end_date
GROUP BY
    a.product_id;

-- SELECT
--     a.product_id,
--     coalesce(round((sum(a.price * b.units) FILTER (WHERE b.purchase_date >= a.start_date
--                 AND b.purchase_date <= a.end_date)::float / max(c.units)::float)::numeric, 2), 0) AS average_price
-- FROM
--     Prices a
--     LEFT JOIN UnitsSold b ON a.product_id = b.product_id
--     LEFT JOIN (
--         SELECT
--             product_id,
--             sum(units) AS units
--         FROM
--             UnitsSold
--         GROUP BY
--             product_id) c ON a.product_id = c.product_id
-- GROUP BY
--     a.product_id;
