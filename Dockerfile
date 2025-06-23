FROM bmauto/roboplc-aarch64:latest
COPY ./target/aarch64-unknown-linux-gnu/release/hello-roboplc /var/roboplc/program/current
