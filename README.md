# Lesson #1
We set up an initial project with basic source code to run an [Actix-Web](https://github.com/actix/actix-web) 
based web service. We will also set up the basics for ANY professional 
software project:
* Source code control with an appropriate `.gitignore` [file](https://docs.github.com/en/get-started/getting-started-with-git/ignoring-files)
* This `README.md` file, which describes a project with instructions on how to run the code

## The Cargo File
The `Cargo.toml` file contains the Rust project description and type along with the dependencies on other libraries and tools:

```toml
[package]
name = "seth_rust_service"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-web = "3"
```

Here we declare that we want to use the [Actix-Web](https://actix.rs/) framework Version 3.
## The Code
The following code is added to the `main.rs` file in the `src` folder to start a web service:

```rust
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

async fn main() -> std::io::Result<()> {
    let bind = "127.0.0.1:3000";

    println!("Starting server at: {}", &bind);
    HttpServer::new(|| App::new().service(index))
        .bind(&bind)?
        .run()
        .await
}

```

### Creating an HTTP handler function
The third line specifies [attribute](https://doc.rust-lang.org/rust-by-example/attribute.html) metadata that specifies the URL pattern to be recognized by the HTTP handler function, `index`.
The function then prints a simple text message containing parameters specified in the pattern to be returned to the caller.
Notice that the `id` parameter MUST be a 32-bit integer and the `name` parameter must be a string. 

### The `main` function
This function starts the HTTP server, passing the handler function, and waits for exit. The server binds itself to the `3000` port.

### Running Server

```sh
cd seth-rust-service
cargo run (or ``cargo watch -x run``)

# Started http server: 127.0.0.1:3000
```

### Testing the server
You can test the server by navigating on a standard browser to [http://127.0.0.1:3000](http://127.0.0.1:3000). You can also use the following command line utilities to test:

Using [HTTPie](https://httpie.org/):
  ```sh
  http GET localhost:3000/123/seth/index.html
  ```

Using cURL:
  ```sh
  curl -S -X GET http://localhost:3000/123/seth/index.html
  ```

Any URL passed to the server that does not conform to the URL pattern will return an [HTTP 404 error](https://www.w3.org/Protocols/rfc2616/rfc2616-sec10.html) (content not found).

### Things to study
#### HTTP
This sample is a demonstration of the [HTTP protocol](https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview), which powers everything on the web (web sites, web services that return JSON to mobile applications, etc.).  No matter what you may want to build, if it is connected to the Internet, you need a service like this one.

#### Microservices
When refined in future iterations, this code is a [microservice](https://microservices.io/). Applications like Facebook, Instagram, LinkedIn, Twitch, and Netflix are all powered by a collection of microservices, each of which is designed to perform a single job.

### Next
We will add database support using [Diesel](https://diesel.rs/) and structure our code using [CLEAN architecture](https://11takanori.medium.com/dependency-inversion-in-rust-web-application-479d54d5242e) to support future iterations of development. [Serde](https://serde.rs/) will be used to serialize and deserialize [JSON](https://docs.serde.rs/serde_json/), the most used data format for microservices.