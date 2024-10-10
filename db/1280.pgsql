-- Time taken: 23:08, 23:12 -> Acc
SELECT
    a.student_id,
    a.student_name,
    b.subject_name,
    count(c.student_id) AS attended_exams
FROM
    Students a
    CROSS JOIN Subjects b
    LEFT JOIN Examinations c ON a.student_id = c.student_id
        AND b.subject_name = c.subject_name
GROUP BY
    a.student_id,
    a.student_name,
    b.subject_name;

