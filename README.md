# Rust Webserver

This is a simple webserver built with [Rocket](https://rocket.rs/) and [Tera](https://keats.github.io/tera/docs/). For more details look at Cargo.

For this test project it produces messages to a Kafka topic, it works with [rdkafka](https://github.com/fede1024/rust-rdkafka).

## Run the server

First setup a Kafka broker and create a topic. Then update this project to match the host and the topic (I still need to create a conf file). You can update
the topic in the main.rs in L34, and the host in producer.rs in L8.

Then run the project with `cargo run` and access to http://127.0.0.1:8000. Submit the form and see the messages that you receive in the Kafka topic, it will
produce the messages as JSON Strings. For now the Kafka key is hardcoded to event1, this is in my TODO.
