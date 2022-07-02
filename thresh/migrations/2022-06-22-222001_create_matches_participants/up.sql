create table matches_participants (
  id SERIAL PRIMARY KEY,
  match_id VARCHAR NOT NULL,
  summoner_id VARCHAR NOT NULL,
  gold_left INTEGER NOT NULL,
  level INTEGER NOT NULL,
  placement INTEGER NOT NULL,
  last_round INTEGER NOT NULL,

  -- Foreign keys
  FOREIGN KEY (match_id) REFERENCES matches(match_id),
  FOREIGN KEY (summoner_id) REFERENCES summoners(puuid)
)