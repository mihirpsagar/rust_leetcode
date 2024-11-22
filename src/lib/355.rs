// Time taken: 23:49, 00:08 -> Acc

use std::collections::{HashMap, VecDeque};

struct Twitter {
    tweets: VecDeque<(i32, i32)>,
    followers: HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        return Self {
            tweets: VecDeque::new(),
            followers: HashMap::new(),
        };
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push_front((user_id, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let arr = if self.followers.contains_key(&user_id) {
            self.followers.get(&user_id).unwrap()
        } else {
            &Vec::new()
        };
        for tweet in self.tweets.iter() {
            if tweet.0 == user_id || arr.binary_search(&tweet.0).is_ok() {
                result.push(tweet.1);
                if result.len() == 10 {
                    break;
                }
            }
        }

        return result;
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(mut arr) = self.followers.get_mut(&follower_id) {
            Self::binary_insert(&mut arr, followee_id);
        } else {
            self.followers.insert(follower_id, vec![followee_id]);
        }
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(mut arr) = self.followers.get_mut(&follower_id) {
            if let Ok(idx) = arr.binary_search(&followee_id) {
                arr.remove(idx);
            }
        }
    }

    pub fn binary_insert(arr: &mut Vec<i32>, target: i32) {
        if arr.is_empty() {
            arr.push(target);
            return;
        }

        let mut left = 0;
        let mut right = arr.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] == target {
                return;
            } else if arr[mid] < target {
                left += 1;
            } else {
                right = mid;
            }
        }

        arr.insert(left, target);
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

fn main() {
    println!("Hello world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
