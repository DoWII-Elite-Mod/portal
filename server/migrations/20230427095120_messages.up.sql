CREATE TABLE message (
  match_relic_id int NOT NULL,
  player_relic_id int NOT NULL,
  body text NOT NULL,
  tick int NOT NULL,
  CONSTRAINT fk_messages_player FOREIGN KEY(player_relic_id) REFERENCES player(relic_id),
  CONSTRAINT fk_messages_match FOREIGN KEY(match_relic_id) REFERENCES match(relic_id)
);
