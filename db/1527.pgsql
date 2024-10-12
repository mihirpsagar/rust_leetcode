-- Time taken: 18:01, 18:02, 18:05 -> Acc
SELECT
    patient_id,
    patient_name,
    conditions
FROM
    Patients
WHERE
    conditions LIKE 'DIAB1%'
    OR conditions LIKE '% DIAB1%';

