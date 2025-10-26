# Quote API

The Quote API.

## Introduction

### Why REST?

Simplicity and speed, plus I had some of the code already written I could leverage as I typically code in Rust. For speed and prototyping REST APIs are great, however, I find it best to use gRPC for streaming and unary calls in the backend where synchronous calls are needed, otherwise async message passing via Kafka or NATs Jetstream is the way to go. The exception, of course, is frontend, where REST and GraphQL are both great depending on the use case.

### Why Rust?

Speed, concurrency, and safety. Rust is a non-garbage collected language with a great compiler and a great standard library. This makes it at minimum 30% faster than Go, Python, Javascript, Java, etc. due to the non-locking nature of the garbage collector and no global interpreter lock. 

I also am the most proficient at Rust, and I have a lot of experience with it thus used it for this project.

### Authentication + Authorization

Actual authentication and authorization would be handled typically via JWT and Outh. This is outside the scope of the demo, but I included a pseudo authentication hook to show that it is possible and how it would be done, and also showcase how to use row level security is Mssql.

### Database

Azure SQL Database. I used the local docker compose file to run the database locally.

## Project Structure

The project shows how a real world project could potentially be structured. The idea is to seperate logic layers to make swapping any part modular and easy. The major layers are:

### Application Binaries: `./bins`
- **api** - The API itself. This is the webserver framework, routes, and middleware that are used to handle requests.

### Crates: `../crates`
- **database** - The code that actually interacts with th specific database engine.
- **types** - The data models that are used to interact with the database. This could also be called `models`, etc.
- **services** - The business logic that is used to interact with the database. This could also be called `use cases`, etc.
- **utils** - The utility code that is used throughout the project. This is not included but could also be an additional library crate, etc.

## Running

### Local

Install Rust and Docker.

- [Install Docker](https://docs.docker.com)
- [Install Rust](https://rust-lang.org/tools/install/)

Make sure to add `rustup` and `cargo` to your path if not automatically added. Then run:

```bash
cargo run --package api --bin api
```

## Database Seeding

Make sure to run the database seeding script before running the API, execute against the running database instance the two `sql` scripts in the `migrations` directory.

## Demo 
