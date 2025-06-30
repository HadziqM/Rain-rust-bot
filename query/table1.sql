BEGIN;

CREATE TABLE IF NOT EXISTS bot_setting (
  name TEXT PRIMARY KEY CHECK (checker IN ('Transfer','Feature','Main'))
  json TEXT NOT NULL,
);

CREATE TABLE IF NOT EXISTS discord (
  discord_id TEXT PRIMARY KEY,
  user_id INT NOT NULL,
  char_id INT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS event (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  discord_id TEXT,
  benefit INT DEFAULT 0,
  bounty_coin INT DEFAULT 0,
  gacha_ticket INT DEFAULT 0,
  gacha_pity INT DEFAULT 0,
  bounty_cd TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  transfer_cd TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (discord_id) REFERENCES discord(discord_id)
);

CREATE TABLE IF NOT EXISTS transfer_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  bounty_id INT NOT NULL,
  discord_id TEXT NOT NULL,
  message_url TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (bounty_id) REFERENCES bounty(id),
  FOREIGN KEY (discord_id) REFERENCES discord(discord_id)
);

CREATE TABLE IF NOT EXISTS assets (
  name TEXT NOT NULL PRIMARY KEY,
  data BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS bounty_category (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  type TEXT NOT NULL CHECK (type IN ('free', 'progression', 'event', 'limited', 'hidden')),
  cooldown INT DEFAULT 0
);

CREATE TABLE IF NOT EXISTS bounty (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  category_id INT NOT NULL,
  bbq INT NOT NULL CHECK (bbq BETWEEN 1 AND 25),
  thumbnail TEXT NOT NULL,
  icon TEXT NOT NULL,
  rules TEXT NOT NULL,
  solo_reward TEXT NOT NULL,
  multi_reward TEXT NOT NULL,
  is_urgent BOOLEAN DEFAULT FALSE,
  FOREIGN KEY (category_id) REFERENCES bounty_category(id),
  FOREIGN KEY (thumbnail) REFERENCES assets(name),
  FOREIGN KEY (icon) REFERENCES assets(name),
  UNIQUE (category_id, bbq)
);

CREATE TABLE IF NOT EXISTS bounty_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  bounty_id INT NOT NULL,
  discord_id TEXT NOT NULL,
  message_url TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (bounty_id) REFERENCES bounty(id),
  FOREIGN KEY (discord_id) REFERENCES discord(discord_id)
);

CREATE TABLE IF NOT EXISTS bounty_progression (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  category_id INT NOT NULL,
  bbq INT NOT NULL DEFAULT 1,
  discord_id TEXT NOT NULL,
  FOREIGN KEY (category_id) REFERENCES bounty_category(id),
  FOREIGN KEY (discord_id) REFERENCES discord(discord_id)
);

CREATE TABLE IF NOT EXISTS bounty_attemp (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  bounty_id INT NOT NULL,
  attemp INT NOT NULL DEFAULT 0,
  discord_id TEXT NOT NULL,
  FOREIGN KEY (bounty_id) REFERENCES bounty(id),
  FOREIGN KEY (discord_id) REFERENCES discord(discord_id)
);

CREATE TABLE IF NOT EXISTS title(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  trigger INT NOT NULL,
  image TEXT NOT NULL,
  setting TEXT NOT NULL,
  role_id TEXT NOT NULL,
  -- use bit flag for add benefit 0 => no trigger, 1 => bounty 10% etc ..
  flag INT DEFAULT 0 CHECK (flag IN (0,1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024)),
  FOREIGN KEY (trigger) REFERENCES bounty(id),
  FOREIGN KEY (image) REFERENCES assets(name)
);

CREATE TABLE IF NOT EXISTS title_history(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title_id INT NOT NULL,
  discord_id TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (title_id) REFERENCES title(id),
  FOREIGN KEY (discord_id) REFERENCES discord(discord_id)
);


CREATE TABLE IF NOT EXISTS gacha(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  -- rarity number are more common so r1 are rarer than r2, sr is rarer than r
  rarity TEXT NOT NULL CHECK (rarity IN ('r1','r2','sr1','sr2','sr3','ssr1','ssr2','ur')),
  item TEXT NOT NULL,
);

-- only store gacha with rarity ssr and up
CREATE TABLE IF NOT EXISTS gacha_history(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  gacha_id INT NOT NULL,
  discord_id TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (gacha_id) REFERENCES gacha(id),
  FOREIGN KEY (discord_id) REFERENCES discord(discord_id) 
);

CREATE TABLE IF NOT EXISTS market_item(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  price INT NOT NULL,
  item TEXT NOT NULL,
);

CREATE TABLE IF NOT EXISTS market_meal(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  meal_id INT NOT NULL,
  level INT NOT NULL CHECK (level IN (1,2,3)),
  -- price per day
  price INT NOT NULL,
);

INSERT INTO bot_setting (name,json) VALUES
  ('Transfer','{
    "cooldown_hour": 168,
    "autoaccept_countdown_mins": 60,
    "allowed_file": {
      "savedata": true,
      "decomyset": true,
      "hunternavi": true,
      "otomoairou": true,
      "partner": true,
      "platedata": true,
      "platebox": true,
      "platemyset": true,
      "rengokudata": true,
      "savemercenary": true
    }
  }');

COMMIT;
