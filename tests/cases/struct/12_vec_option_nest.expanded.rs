#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct UserProfile {
    name: String,
    bio: String,
}
struct Stats {
    follower_count: i64,
    followee_count: i64,
}
struct User {
    id: String,
    profile: Vec<UserProfile>,
    stats: Option<Stats>,
}
