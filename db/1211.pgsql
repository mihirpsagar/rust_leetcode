-- Time taken: 23:49, 00:02 -> Wrong, 00:06 -> Wrong, 00:07 -> Acc
SELECT
    query_name,
    ROUND(SUM(rating::numeric / position) / COUNT(rating), 2) AS quality,
    ROUND(100.00 * COUNT(rating) FILTER (WHERE rating < 3) / COUNT(rating), 2) AS poor_query_percentage
FROM
    Queries
WHERE
    query_name IS NOT NULL
GROUP BY
    query_name;

