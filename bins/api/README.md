# Quote API

The quote REST API.

### Why REST?

Simplicity and speed, plus I had some of the code already written I could leverage as I typically code in Rust. For speed and prototyping REST APIs are great, however, I find it best to use gRPC for streaming and unary calls in the backend where synchronous calls are needed, otherwise async message passing via Kafka or NATs Jetstream is the way to go. The exception, of course, is frontend, where REST and GraphQL are both great depending on the use case.

### Why Rust?

Speed, concurrency, and safety. 
