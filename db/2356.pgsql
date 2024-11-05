-- Time taken: 22:04, 22:05 -> Acc
SELECT
    teacher_id,
    count(DISTINCT subject_id) AS cnt
FROM
    Teacher
GROUP BY
    teacher_id;

