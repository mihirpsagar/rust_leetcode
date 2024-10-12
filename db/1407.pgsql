-- Time taken: 10:40, 10:44 -> Wrong, 10:45 -> Acc
SELECT
    name,
    travelled_distance
FROM (
    SELECT
        a.id,
        a.name,
        coalesce(sum(b.distance), 0) AS travelled_distance
    FROM
        Users a
    LEFT JOIN Rides b ON a.id = b.user_id
GROUP BY
    a.id,
    a.name
ORDER BY
    sum(b.distance) DESC NULLS LAST,
    a.name ASC);

