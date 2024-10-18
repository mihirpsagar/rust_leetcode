-- Time taken: 11:21, 11:24, 11:29 -> Acc
SELECT
    a.name,
    b.balance
FROM
    Users a
    INNER JOIN (
        SELECT
            account,
            sum(amount) AS balance
        FROM
            Transactions
        GROUP BY
            account
        HAVING
            sum(amount) > 10000) b ON a.account = b.account;

