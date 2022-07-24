export interface MatchParticipantAugmentI {
  items: {
    name_id: string;
  };
}

export interface MatchParticipantTraitI {
  style: number;
  tier_current: number;
  tier_total: number;
  num_units: number;
  traits: {
    display_name: string;
    trait_id: string;
  };
}

export interface MatchParticipantUnitI {
  characters: {
    display_name: string;
    img: string;
    tier: number;
  };
  tier: number;
}

export interface MatchParticipantI {
  gold_left: number;
  level: number;
  matches_participants_augments: MatchParticipantAugmentI[];
  matches_participants_traits: MatchParticipantTraitI[];
  matches_participants_units: MatchParticipantUnitI[];
  placement: number;
  summoners: {
    name: string;
  };
}

export interface MatchI {
  game_datetime: Date;
  matches_participants: MatchParticipantI[];
  region: string;
}
