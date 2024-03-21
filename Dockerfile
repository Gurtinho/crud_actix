FROM rust:latest as build

ENV APP=app
ENV url="postgres://postgres:crud_actix@localhost:5455/crud_actix?schema=public"

# create a new empty shell project
RUN USER=root cargo new --bin ${APP}
WORKDIR /${APP}

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./.env ./.env

# run command to generate migrations
RUN cargo install sqlx-cli
# --database-url="postgres://postgres:crud_actix@database:5455/crud_actix?schema=public"
# RUN sqlx migrate run --source src/http/databases/migrations

# build for release
RUN rm ./target/release/deps/${APP}*
RUN cargo build --release

# our final base
FROM rust:latest

# copy the build artifact from the build stage
COPY --from=build /${APP}/target/release/${APP} .

EXPOSE 9000

# set the startup command to run your binary
CMD ["./${APP}"]