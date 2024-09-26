-- Time taken: 23:08, 23:18 -> Acc
SELECT
    max(num) AS num
FROM (
    SELECT
        num
    FROM
        MyNumbers
    GROUP BY
        num
    HAVING
        count(num) = 1);

