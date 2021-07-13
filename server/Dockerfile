# FROM rustlang/rust:nightly as builder
# WORKDIR /server
# COPY . .
# RUN cargo build --release --bin server

# FROM rustlang/rust:nightly as runtime
# WORKDIR /server
# COPY --from=builder /server/target/release/server /usr/local/bin
# EXPOSE 8000
# ENTRYPOINT [ "/usr/local/bin/snake-server" ]

FROM rustlang/rust:nightly as planner
WORKDIR /battlesnake
# We only pay the installation cost once,
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning
# the cargo-chef version with `--version X.X.X`
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rustlang/rust:nightly as cacher
WORKDIR /battlesnake
RUN cargo install cargo-chef
COPY --from=planner /battlesnake/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rustlang/rust:nightly as builder
WORKDIR /battlesnake
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /battlesnake/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin server

FROM rustlang/rust:nightly as runtime
WORKDIR /battlesnake
COPY --from=builder /battlesnake/target/release/server /usr/local/bin/snake
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/snake"]
