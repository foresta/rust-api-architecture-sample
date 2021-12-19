# Rust API Server Architecture Sample

This is sample code of layered architecture in rust.


## Layer

- domains
    - Contains domain logic 
- infrastructures
    - To access external resources such as Database and HTTP Requests
- usecases
    - Call the domain logic and make adjustments to meet the required use cases.
- server
    - Interface such as HTTP Server .

