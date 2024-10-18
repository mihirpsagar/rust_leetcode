-- Time taken: 22:19, 22:20 -> Acc
SELECT
    tweet_id
FROM
    Tweets
WHERE
    LENGTH(content) > 15;

