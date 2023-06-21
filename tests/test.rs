//! tests/test.rs

use std::{io, time::{Instant}};
use redis::Commands;
use redis_testing::connection::connect;
use redis_testing::connection::GEOSET_NAME;
use rand::Rng;

#[tokio::test]
async fn test_num_geoadd_and_check_length () {
  let mut rng = rand::thread_rng();
  // Create connection to redis server
  let mut conn = connect();

  let mut num: String = String::new();

  io::stdin().read_line(&mut num).unwrap();

  let num: i64 = num.trim().parse().unwrap();

  let start = Instant::now();

  let mut lat: f64;
  let mut long: f64;

  let mut i = 0;
  while i < num {
    lat = rng.gen_range(-85.05..85.05);
    long = rng.gen_range(-180.0..180.0);
    let _: () = conn.geo_add(GEOSET_NAME,(long, lat, format!("loc{}", i)))
      .unwrap_or_else(|_| panic!("failed to insert on number {}", i));
    i += 1;
  }

  let x: i64 = conn.zcard(GEOSET_NAME).expect("Failed to get length of inserted set");
  assert_eq!(&num, &x);


  let duration = start.elapsed();
  println!("Time elapsed in test_100_geoadd_and_check_length() is : {:?}", duration);

  let _: () = conn.zremrangebyrank(GEOSET_NAME, 0, num.try_into().unwrap()).expect("Failed to execute sorted set deletion");
}


#[tokio::test]
async fn test_num_random_geoadd_and_check_length () {
  let start = Instant::now();

  // Create connection to redis server
  let mut conn = connect();

  let mut num: String = String::new();

  io::stdin().read_line(&mut num).unwrap();

  let num: i64 = num.trim().parse().unwrap();

  let mut i = 0;
  while i < num {
    let _: () = conn.geo_add(GEOSET_NAME,("12", "12", format!("loc{}", i)))
      .unwrap_or_else(|_| panic!("failed to insert on number {}", i));
    i += 1;
  }

  let x: i64 = conn.zcard(GEOSET_NAME).expect("Failed to get length of inserted set");
  assert_eq!(&num, &x);


  let duration = start.elapsed();
  println!("Time elapsed in test_100_geoadd_and_check_length() is : {:?}", duration);

  let _: () = conn.zremrangebyrank(GEOSET_NAME, 0, num.try_into().unwrap()).expect("Failed to execute sorted set deletion");
}