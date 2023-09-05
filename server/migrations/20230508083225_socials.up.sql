-- Add up migration script here
CREATE TABLE socials (
  relic_id int UNIQUE NOT NULL,
  twitch_id text UNIQUE,
  youtube_id text UNIQUE,
  discord_id text UNIQUE,
  forum_id int UNIQUE,
  CONSTRAINT fk_socials_player FOREIGN KEY(relic_id) REFERENCES player(relic_id)
)
