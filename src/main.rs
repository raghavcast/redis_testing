//! src/main.rs

use redis_testing::connection::connect;
use redis_testing::connection::GEOSET_NAME;
use redis_testing::sortnot::sort_or_not;
use redis_testing::hashing::{to_hash, from_hash};

fn main() {
    let mut conn = connect();
    sort_or_not(conn);
}
