FROM rust:latest
WORKDIR /usr/src/McNugget_Countdown
COPY . .
RUN cargo install --path .
CMD ["McNugget_Countdown"]
