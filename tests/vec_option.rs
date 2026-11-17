#![allow(dead_code)]

use nest_struct::nest_struct;

#[nest_struct]
struct User {
    id: String,
    profile: Vec<
        nest! {
            name: String,
            bio: String,
        },
    >,
    stats: Option<
        Stats! {
            follower_count: i64,
            followee_count: i64,
        },
    >,
}
