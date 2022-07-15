CREATE TABLE matches (
  match_id VARCHAR NOT NULL PRIMARY KEY,
  game_datetime BIGINT NOT NULL,
  game_length INT NOT NULL,
  tft_set_core_name VARCHAR NOT NULL,
  region VARCHAR
)