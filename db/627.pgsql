-- Time taken: 23:21, 23:23 -> Acc
UPDATE
    Salary
SET
    sex = CASE WHEN sex = 'f' THEN
        'm'
    ELSE
        'f'
    END;

