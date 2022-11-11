# speed-monitor

Call `fast.com` speed test, store data in sqlite.

## Usage

- `docker buildx build --progress=plain --platform linux/amd64 -t speedy .`
  - you will have trouble building rust's c deps in musl on other arch
- run the app

```
docker run \
  --rm \
  -it \
  -v "$PWD/speedy.db:/app/speedy.db" \
  speedy
```

## Configuration

All config is via the ENV, or `--env KEY=VALUE` for the docker command above.

| **key**      | **default value**             | **description**                                                                     |
| ------------ | ----------------------------- | ----------------------------------------------------------------------------------- |
| CRON         | `0 0 0 * * *`                 | cron format follows https://docs.rs/cron/latest/cron/#example                       |
| DATABASE_URL | `sqlite://speedy.db?mode=rwc` | SQLite url                                                                          |
| MODE         | `production`                  | "development" or "production". "development" skips slow operations & uses fake data |
