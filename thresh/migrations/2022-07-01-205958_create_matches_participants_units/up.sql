create table matches_participants_units (
  id SERIAL PRIMARY KEY,
  match_participant_id INTEGER NOT NULL,
  unit_id VARCHAR NOT NULL,
  rarity INTEGER NOT NULL,
  tier INTEGER NOT NULL,

  -- Foreign keys
  FOREIGN KEY (match_participant_id) REFERENCES matches_participants(id),
  FOREIGN KEY (unit_id) REFERENCES characters(character_id)
)