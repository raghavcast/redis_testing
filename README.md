# Redis Testing
This repository is for me to test out implementations of redis when using rust.

## Repo Structure

- src
  -  main.rs - holds the main function
  -  connection.rs - functions to establish connection to redis server. Reads the info from the [.env](.env) file
  -  attempts.rs - holds functions that I wrote to understand rust and how it interacts with the Redis API
  -  hashing.rs - functions that deal with geohashing
  -  lib.rs - library file as specified in one of the rust tutorials that I learned from
  -  ***sortnot.rs*** - Holds the function that I wrote in order to test the location update functionality
- tests
  - test.rs - Test file that I wrote but didn't use all that much
- .env - environment file      
