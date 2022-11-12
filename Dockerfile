FROM rustlang/rust:nightly-alpine as build
WORKDIR /app
RUN apk add --no-cache libc-dev musl-dev
COPY . .
RUN ls -al && cargo install --jobs 1 --path .
RUN echo $(which speed-monitor) && mv $(which speed-monitor) ./

FROM node:alpine
WORKDIR /app
RUN apk add chromium
RUN npm i -g fast-cli
COPY --from=build /app/speed-monitor /app/
CMD ["speed-monitor"]
