CREATE TABLE characters (
  character_id VARCHAR NOT NULL PRIMARY KEY,
  img VARCHAR,
  tier INTEGER,
  rarity INTEGER,
  display_name VARCHAR NOT NULL,
  square_icon_path VARCHAR NOT NULL
)