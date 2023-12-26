FROM rust:1.73-slim-buster AS builder

RUN USER=root cargo new --bin oasis-log-collector
WORKDIR ./oasis-log-collector
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/oasis_log_collector*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /oasis-log-collector/target/release/oasis-log-collector ${APP}/oasis-log-collector

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["./oasis-log-collector", "--port", "3000"]
