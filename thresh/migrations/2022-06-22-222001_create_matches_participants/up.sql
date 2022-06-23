create table matches_participants (
  id SERIAL PRIMARY KEY,
  match_id VARCHAR NOT NULL,
  summoner_id VARCHAR NOT NULL,

  -- Foreign keys
  FOREIGN KEY (match_id) REFERENCES matches(match_id),
  FOREIGN KEY (summoner_id) REFERENCES summoners(id)
)