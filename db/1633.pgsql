-- Time taken: 13:29, 13:35 -> Acc
SELECT
    contest_id,
    round(count(user_id) * 100 /(
            SELECT
                COUNT(user_id)
            FROM Users)::numeric, 2) AS percentage
FROM
    Register
GROUP BY
    contest_id
ORDER BY
    percentage DESC,
    contest_id ASC;

