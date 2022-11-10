create table if not exists speed_samples
(
  id integer primary key autoincrement,
  timestamp default (datetime('now','localtime')),
  buffer_bloat float not null,
  downloaded i64,
  download_speed i64,
  latency i64,
  uploaded i64,
  upload_speed i64,
  user_ip text,
  user_location text
)
