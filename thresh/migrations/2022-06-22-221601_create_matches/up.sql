CREATE TABLE matches (
  match_id VARCHAR NOT NULL PRIMARY KEY,
  game_datetime TIMESTAMP NOT NULL,
  game_length INTEGER NOT NULL,
  tft_set_core_name VARCHAR NOT NULL,
  region VARCHAR
)