-- Time taken: 20:45, 20:50 -> Acc
SELECT
    customer_id,
    count(customer_id) AS count_no_trans
FROM
    Visits
WHERE
    visit_id NOT IN (
        SELECT
            visit_id
        FROM
            Transactions)
GROUP BY
    customer_id;

