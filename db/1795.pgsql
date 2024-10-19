-- Time taken: 23:38, 23:48 -> Acc
SELECT
    product_id,
    store,
    price
FROM (
    SELECT
        product_id,
        unnest(ARRAY['store1', 'store2', 'store3']) AS store,
        unnest(ARRAY[store1, store2, store3]) AS price
    FROM
        Products)
WHERE
    price IS NOT NULL;

