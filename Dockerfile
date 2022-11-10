FROM rust:alpine as build
WORKDIR /app
COPY . .
RUN cargo install --jobs 1 --path .
RUN echo $(which speed-monitor) && mv $(which speed-monitor) ./

FROM node:alpine
WORKDIR /app
RUN apk add chromium
RUN npm i -g fast-cli
COPY --from=build /app/speed-monitor /app/
CMD ["speed-monitor"]
