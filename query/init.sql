create table if not exists discord_register(
	id serial,
	discord_id varchar(32) unique,
  user_id int,
  created_at timestamp without time zone default now(),
	primary key(id)
);

CREATE TABLE IF NOT EXISTS discord(
	id SERIAL,
	char_id INT NOT NULL,
	discord_id VARCHAR(32) unique,
	bounty INT,
  newbie BOOLEAN DEFAULT true,
  gacha INT DEFAULT 0,
  pity INT DEFAULT 0,
  latest_bounty VARCHAR(10),
  latest_bounty_time BIGINT DEFAULT 0,
  boostcd BIGINT DEFAULT 0,
  transfercd BIGINT DEFAULT 0,
	PRIMARY KEY(id),
      CONSTRAINT fk_discord
      FOREIGN KEY(discord_id) 
	  REFERENCES discord_register(discord_id)
);
create table if not exists bounty(
	id serial,
	title varchar(255) not null,
	explain varchar(255),
	solo_point int not null,
	multi_point int not null,
	solo_ticket int not null,
	multi_ticket int not null,
	cooldown int not null,
	primary key(id)
);
create table if not exists submitted(
	id serial,
	bbq varchar(255),
	type_b int not null default 1,
	title varchar(255),
	cid int not null default 0,
	team text default 'none',
    cname text default 'none',
    uname text default 'none',
	t_submit int not null,
	avatar varchar(255),
	url_i varchar(255),
	primary key(id)
);
drop table if exists mezfes;

create table mezfes(
	id serial,
	discord_id varchar(32),
    Panic_Honey int not null,
    Guuku_Scoop int not null,
    Dokkan_Battle_Cats int not null,
    Nyanrendo int not null,
    Uruki_Pachinko int not null,
    total int not null,
	primary key(id),
    CONSTRAINT fk_discord
      FOREIGN KEY(discord_id) 
	  REFERENCES discord(discord_id)
);
ALTER TABLE distribution ADD COLUMN IF NOT EXISTS bot boolean DEFAULT false;
ALTER TABLE discord ADD COLUMN IF NOT EXISTS transfercd BIGINT DEFAULT 0;
