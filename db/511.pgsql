-- Time taken: 23:50, 23:52
SELECT
    player_id,
    min(event_date) AS first_login
FROM
    Activity
GROUP BY
    player_id
ORDER BY
    player_id ASC;

