# Installation instructions (proceed in order)

If you are working under windows it is advisable to get a Linux Host going under WSL 2 because tooling setup is generally (imho) a smoother experience. Instructions for Windows systems are provided too though.

1. Install Rust toolchain
Navigate to https://rustup.rs and install the toolchain by following the instructions on that site.

   If you are working with Windows follow setup instructions from here too: https://rust-lang.github.io/rustup/installation/windows-msvc.html (**not optional**)

   If you are working with Linux (either Host or WSL2) make sure you have build-tools available. Command for Ubuntu:
   ```
   sudo apt install build-essential
   ```
   Might be different if you are using another Linux distribution.

   If you are working with Mac OS make sure to have XCode toolchain/build tools installed.


1. Install Docker

   Go to https://www.docker.com/ and follow setup instructions for your operating system.


1. Install the SQLX toolchain
```
cargo install sqlx-cli
```


1. Fire up database

```
docker compose up -d
```


1. Initialize database

```
sqlx database create
sqlx migrate run
```  
   
   If you want to reset the database and start from scratch do the following:
```
sqlx database drop
sqlx migrate run
```

1. Calculate ratings for games played
docker exec -it portal-server-1 ./ratings

# Public API Routes

## Players

- GET /players - List of players (paginated)
- GET /players/:id - Single player details
- POST /players - Add new player details

## Games

- GET /games - List of games (paginated)
- GET /games/:id - Single game details

## Game result upload

- POST /reports - Report results of a single game
