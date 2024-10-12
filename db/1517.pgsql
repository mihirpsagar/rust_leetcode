-- Time taken: 17:26, 17:31 -> Acc
SELECT
    user_id,
    name,
    mail
FROM
    Users
WHERE
    mail ~ '^[a-zA-Z]+[a-zA-Z0-9_.-]*@leetcode\.com$';

