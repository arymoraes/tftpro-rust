create table matches_participants_traits (
  id SERIAL PRIMARY KEY,
  match_participant_id INTEGER NOT NULL,
  trait_id VARCHAR NOT NULL,
  num_units INTEGER NOT NULL,
  tier_current INTEGER NOT NULL,
  tier_total INTEGER NOT NULL,
  style INTEGER NOT NULL,

  -- Foreign keys
  FOREIGN KEY (match_participant_id) REFERENCES matches_participants(id),
  FOREIGN KEY (trait_id) REFERENCES traits(trait_id)
)