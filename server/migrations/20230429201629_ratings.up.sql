-- Add up migration script here
CREATE TABLE rating (
  relic_id int NOT NULL,
  timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  rating DOUBLE PRECISION NOT NULL DEFAULT 1500,
  deviation DOUBLE PRECISION NOT NULL DEFAULT 350,
  volatility DOUBLE PRECISION NOT NULL DEFAULT 0.06,
  CONSTRAINT fk_ratings_player FOREIGN KEY(relic_id) REFERENCES player(relic_id)
);
