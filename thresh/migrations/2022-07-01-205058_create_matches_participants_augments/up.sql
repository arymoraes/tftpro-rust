create table matches_participants_augments (
  id SERIAL PRIMARY KEY,
  match_participant_id INTEGER NOT NULL,
  augment_id INTEGER NOT NULL,

  -- Foreign keys
  FOREIGN KEY (match_participant_id) REFERENCES matches_participants(id),
  FOREIGN KEY (augment_id) REFERENCES items(id)
)