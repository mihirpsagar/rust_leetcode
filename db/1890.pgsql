-- Time taken: 22:44, 22:50 -> Acc
SELECT
    user_id,
    last_stamp
FROM (
    SELECT
        user_id,
        max(time_stamp) FILTER (WHERE date_part('year', time_stamp) = '2020') AS last_stamp
    FROM
        Logins
    GROUP BY
        user_id)
WHERE
    last_stamp IS NOT NULL;

