create table matches_participants_unit_items (
  id SERIAL PRIMARY KEY,
  match_participant_unit_id INTEGER NOT NULL,
  item_id INTEGER NOT NULL,

  -- Foreign keys
  FOREIGN KEY (match_participant_unit_id) REFERENCES matches_participants_units(id),
  FOREIGN KEY (item_id) REFERENCES items(id)
)