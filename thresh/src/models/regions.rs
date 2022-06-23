enum SubRegions {
    BR1,
    EUN1,
    EUW1,
    JP1,
    KR,
    LA1,
    LA2,
    NA1,
    OC1,
    TR1,
    RU,
}

enum Regions {
    Americas,
    Europe,
    Asia,
}

impl From<SubRegions> for Regions {
    fn from(sub_region: SubRegions) -> Regions {
        match sub_region {
            SubRegions::BR1 => Regions::Americas,
            SubRegions::EUN1 => Regions::Europe,
            SubRegions::EUW1 => Regions::Europe,
            SubRegions::JP1 => Regions::Asia,
            SubRegions::KR => Regions::Asia,
            SubRegions::LA1 => Regions::Asia,
            SubRegions::LA2 => Regions::Asia,
            SubRegions::NA1 => Regions::Americas,
            SubRegions::OC1 => Regions::Europe,
            SubRegions::TR1 => Regions::Europe,
            SubRegions::RU => Regions::Europe,
        }
    }
}

impl From<SubRegions> for String {
    fn from(sub_region: SubRegions) -> String {
        match sub_region {
            SubRegions::BR1 => "BR1".to_string(),
            SubRegions::EUN1 => "EUN1".to_string(),
            SubRegions::EUW1 => "EUW1".to_string(),
            SubRegions::JP1 => "JP1".to_string(),
            SubRegions::KR => "KR".to_string(),
            SubRegions::LA1 => "LA1".to_string(),
            SubRegions::LA2 => "LA2".to_string(),
            SubRegions::NA1 => "NA1".to_string(),
            SubRegions::OC1 => "OC1".to_string(),
            SubRegions::TR1 => "TR1".to_string(),
            SubRegions::RU => "RU".to_string(),
        }
    }
}

impl From<Regions> for String {
    fn from(region: Regions) -> String {
        match region {
            Regions::Americas => "americas".to_string(),
            Regions::Europe => "europe".to_string(),
            Regions::Asia => "asia".to_string(),
        }
    }
}
