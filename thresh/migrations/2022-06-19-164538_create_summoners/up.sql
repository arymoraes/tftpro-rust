CREATE TABLE summoners (
  id VARCHAR NOT NULL PRIMARY KEY,
  account_id VARCHAR NOT NULL,
  puuid VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  summoner_level INTEGER NOT NULL,
  revision_date BIGINT NOT NULL,
  profile_icon_id INTEGER NOT NULL,
  region VARCHAR,
  league_id VARCHAR NOT NULL,

  -- Foreign keys
  FOREIGN KEY (league_id) REFERENCES leagues(league_id)
)