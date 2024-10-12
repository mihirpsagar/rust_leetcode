-- Time taken: 13:49, 13:53 -> Acc
SELECT
    sell_date,
    count(DISTINCT product) AS num_sold,
    string_agg(DISTINCT product, ',' ORDER BY product ASC) AS products
FROM
    Activities
GROUP BY
    sell_date
ORDER BY
    sell_date ASC;

