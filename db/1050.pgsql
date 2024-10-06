-- Time taken: 20:04, 20:07 -> Acc
SELECT
    actor_id,
    director_id
FROM
    ActorDirector
GROUP BY
    actor_id,
    director_id
HAVING
    count(*) > 2;

