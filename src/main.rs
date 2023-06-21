//! src/main.rs

use redis_testing::connection::connect;
use redis_testing::sortnot::sort_or_not;

fn main() {
    let conn = connect();
    sort_or_not(conn);
}
