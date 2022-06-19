CREATE TABLE summoners (
  id VARCHAR NOT NULL PRIMARY KEY,
  account_id VARCHAR,
  puuid VARCHAR,
  name VARCHAR,
  summoner_level INTEGER,
  revision_date BIGINT,
  profile_icon_id INTEGER,
  region VARCHAR,
  league_id VARCHAR,

  -- Foreign keys
  FOREIGN KEY (league_id) REFERENCES leagues(league_id)
)