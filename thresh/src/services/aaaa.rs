// use std::{thread, time::Duration};

// use colored::Colorize;
// use diesel::PgConnection;
// use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

// use crate::{
//     models::{
//         item::Item,
//         league_match::{
//             augments::NewMatchParticipantAugment,
//             dto::{MatchDto, MatchDtoParticipant},
//             league_match::NewMatch,
//             match_participant::NewMatchParticipant,
//         },
//         regions::{Regions, SubRegions},
//         summoner::Summoner,
//     },
//     pool::get_connection_pool,
// };

// use super::{get_api_key, request::RequestOptions, summoner::create_summoner_from_puuid};
