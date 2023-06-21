//! src/sortnot.rs

use std::{io::{self, Write}, time::Instant, process::exit};
use redis::{Commands};
use super::connection::GEOSET_NAME;
use rand::Rng;
use super::hashing::{from_hash, to_hash};

pub fn sort_or_not(mut conn: redis::Connection) {
  let mut rng = rand::thread_rng();

  let mut num: String = String::new();

  io::stdin().read_line(&mut num).unwrap();

  let num: i64 = num.trim().parse().unwrap();

  //
  // Inserting blanks
  //
  let start = Instant::now();

  let x = (0..num).map(|x| (0, 0, format!("loc{}", x))).collect::<Vec<(i64, i64, String)>>();
  //println!("{:?}", x);
  // while i < num {
  //   let _: () = conn.geo_add(GEOSET_NAME,(0, 0, format!("loc{}", i)))
  //     .expect(&format!("failed to insert on number {}", i));
  //   i += 1;
  // }

  let _: () = conn.geo_add(GEOSET_NAME, x)
      .expect("failed to initialize");

  let duration = start.elapsed();
  println!("Time elapsed in main() while setting up set: {:?}", duration);

  //
  // making list of random coords
  //
  let mut i = 0;
  let mut lat: f64;
  let mut long: f64;
  let mut rand_list: Vec<(f64, f64, String)> = Vec::new();
  while i < num {
    lat = rng.gen_range(-85.05..85.05);
    long = rng.gen_range(-180.0..180.0);
    rand_list.push((long, lat, format!("loc{}", i)));
    i += 1;
  }

  //
  // Unsorted update
  //

  let start = Instant::now();

  // for coord in &rand_list {
  //     let _: () = conn.geo_add(GEOSET_NAME,coord)
  //       .expect("failed to insert");
  // }
  let _: () = conn.geo_add(GEOSET_NAME, &rand_list)
      .expect("failed to insert");

  let duration = start.elapsed();
  println!("Time elapsed in main() without sorting is : {:?}", duration);
  
  print!("Continue? (y/n): ");
  io::stdout().flush().unwrap();
  let mut inp = String::new();
  io::stdin().read_line(&mut inp).unwrap();

  let inp = inp.trim();

  match inp {
    "n" =>   exit(0),
    _ => {
      println!("okay");
    },
  }
  
  //
  // Sorted update
  //

  let start = Instant::now();
  
  // Sorting start

  let mut hash_list = rand_list.iter().map(|(x, y, loc)| (to_hash(x, y), loc)).collect::<Vec<(String, &String)>>();

  hash_list.sort_by(|(a, _),(b, _)| a.cmp(b));

  let new_list = hash_list.iter()
    .map(|(x, loc)| {
      let coo = from_hash(x);
      (coo.x, coo.y, *loc)
    }).collect::<Vec<(f64, f64, &String)>>();

// Sorting end

  let start_nosort = Instant::now();

  let _: () = conn.geo_add(GEOSET_NAME,  &new_list)
      .expect("failed to insert");
  // for coord in &rand_list {        
  //     let _: () = conn.geo_add(GEOSET_NAME,coord)
  //       .expect("failed to insert");
  // }

  let duration_sort = start.elapsed();
  let duration_nosort = start_nosort.elapsed();
  println!("Time elapsed in main() with sorting (including sorting time) is : {:?}", duration_sort);
  println!("Time elapsed in main() with sorting (not including sorting time) is: {:?}", duration_nosort);
  //println!("compare: {:?} {:?}", new_list[0], rand_list[0]);

  print!("Delete entries? (y/n): ");
  io::stdout().flush().unwrap();
  let mut inp = String::new();
  io::stdin().read_line(&mut inp).unwrap();

  let inp = inp.trim();

  match inp {
    "y" =>  {
      let _:() = conn.zremrangebyrank(GEOSET_NAME, 0, num.try_into().unwrap())
      .expect("failed to delete the entries");
      println!("deleted entries");
    },
    _ => println!("okay"),
  }
}
