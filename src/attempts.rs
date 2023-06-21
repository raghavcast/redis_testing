//! src/attempts.rs

use super::connection::connect;
use std::collections::BTreeMap;
use redis::{Commands};

fn _hash() {
let mut conn = connect();

let mut driver: BTreeMap<String, String> = BTreeMap::new();
let prefix = "redis-driver";
driver.insert(String::from("name"), String::from("redis-rs"));
driver.insert(String::from("version"), String::from("0.19.0"));
driver.insert(
    String::from("repo"),
    String::from("https://github.com/mitsuhiko/redis-rs")
);
let _: () = redis::cmd("HSET")
    .arg(format!("{}:{}", prefix, "rust"))
    .arg(driver)
    .query(& mut conn)
    .expect("failed to execute HSET");

let info: BTreeMap<String, String> = redis::cmd("HGETALL")
    .arg(format!("{}:{}", prefix, "rust"))
    .query(&mut conn)
    .expect("failed to execute HGETALL");
println!("info for rust redis driver: {:?}", info);

let _: () = conn
    .hset_multiple(
        format!("{}:{}", prefix, "go"),
        &[
            ("name", "go-redis"),
            ("version", "8.4.6"),
            ("repo", "https://github.com/go-redis/redis"),
        ],
    )
    .expect("failed to eceute HSET");

let repo_name: String = conn
    .hget(format!("{}:{}", prefix, "go"), "repo")
    .expect("HGET failed");
println!("go redis driver repo name: {:?}", repo_name);
}

fn _basics() {
let mut conn = connect();
let _bar = "hello world";

let _: () = redis::cmd("SET")
    .arg("foo")
    .arg("bar")
    .query(&mut conn)
    .expect("failed to execute SET for 'foo'");

let bar: String = redis::cmd("GET")
    .arg("foo")
    .query(&mut conn)
    .expect("failed to execute GET for 'foo'");
println!("value for 'foo': {}", bar);

let _: () = conn
    .incr("counter", 2)
    .expect("failed to execute INCR for 'counter'");

let val: i32 = conn
    .get("counter")
    .expect("failed to execute GET for 'counter'");
println!("counter = {}", val);
}


