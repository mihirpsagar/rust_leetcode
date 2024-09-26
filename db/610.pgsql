-- Time taken: 22:31, 22:35 -> Acc, 22:38 -> Optimized
SELECT
    x,
    y,
    z,
    CASE WHEN x + y <= z
        OR y + z <= x
        OR z + x <= y THEN
        'No'
    ELSE
        'Yes'
    END AS triangle
FROM
    Triangle;

