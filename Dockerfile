# NB: This is not a production-grade Dockerfile.

#################
## build stage ##
#################
FROM rust:buster AS builder
WORKDIR /code



# Download crates-io index and fetch dependency code.
# This step avoids needing to spend time on every build downloading the index
# which can take a long time within the docker context. Docker will cache it.
RUN USER=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch

# copy app files
COPY src src

# compile app
RUN cargo build --release


###############
## spa setup ##
###############
FROM node:19-alpine AS spa_builder
WORKDIR /usr/src/app
COPY ./vue-project ./
RUN npm install
RUN npm run build


###############
## run stage ##
###############
FROM debian:buster
RUN apt update && apt -y install postgresql-client
WORKDIR /app

# copy server binary from build stage
COPY --from=builder /code/target/release/vue_actix_template vue_actix_template

# copy spa from build stage
COPY --from=spa_builder /usr/src/app/dist spa


# set user to non-root unless root is required for your app
USER 1001

# indicate what port the server is running on
EXPOSE 8080

# run server
CMD [ "/app/vue_actix_template" ]