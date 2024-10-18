-- Time taken: 22:34, 22:36 -> Acc
SELECT
    date_id,
    make_name,
    count(DISTINCT (lead_id)) AS unique_leads,
    count(DISTINCT (partner_id)) AS unique_partners
FROM
    DailySales
GROUP BY
    date_id,
    make_name;

