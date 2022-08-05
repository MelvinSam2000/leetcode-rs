use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct UserId(i32);
struct TweetId(i32);

impl From<i32> for UserId {
    fn from(user_id: i32) -> Self {
        Self(user_id)
    }
}

impl From<i32> for TweetId {
    fn from(tweet_id: i32) -> Self {
        Self(tweet_id)
    }
}

#[derive(Default)]
pub struct Twitter {
    tweets: Vec<(UserId, TweetId)>,
    followers: HashMap<UserId, HashSet<UserId>>,
}

impl Twitter {
    pub fn new() -> Self {
        Self::default()
    }

    // O(1)
    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push((user_id.into(), tweet_id.into()));
    }

    // O(n) where n - tweets length
    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut feeds = vec![];
        for tweet in self.tweets.iter().rev() {
            let &(UserId(u_id), TweetId(t_id)) = tweet;
            if user_id == u_id {
                feeds.push(t_id);
            } else if let Some(set) = self.followers.get(&user_id.into()) {
                if set.contains(&u_id.into()) {
                    feeds.push(t_id);
                }
            }
            if feeds.len() == 10 {
                break;
            }
        }
        feeds
    }

    // O(1)
    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        let follower_id = follower_id.into();
        self.followers
            .entry(follower_id)
            .or_insert_with(HashSet::new);
        self.followers
            .get_mut(&follower_id)
            .unwrap()
            .insert(followee_id.into());
    }

    // O(1)
    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(set) = self.followers.get_mut(&follower_id.into()) {
            set.remove(&followee_id.into());
        }
    }
}
