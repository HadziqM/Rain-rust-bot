create table if not exists discord(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  discord_id varchar(32) unique,
  user_id int not null,
  char_id int not null,
  created_at timestampz default now()
);

