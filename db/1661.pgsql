-- Time taken: 17:37, 17:42 -> Acc
SELECT
    machine_id,
    round((sum(time_taken) / count(machine_id))::numeric, 3) AS processing_time
FROM (
    SELECT
        machine_id,
        max(timestamp) - min(timestamp) AS time_taken
    FROM
        Activity
    GROUP BY
        machine_id,
        process_id)
GROUP BY
    machine_id;

