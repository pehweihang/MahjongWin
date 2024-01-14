use std::{
    collections::{hash_map, HashMap},
    str::Matches,
};

use crate::{
    hand::{ConcealedTiles, Hand},
    meld::{Meld, MeldType},
    tile::{Flower, FlowerValue, Suit, Tile, TileValue, Wind},
};

pub type ScoreTai = HashMap<Score, u8>;

#[derive(Debug)]
pub struct Hu {
    melds: Vec<Meld>,
    scores: Vec<Score>,
    tai: u8,
}

impl Hu {
    pub fn new(melds: Vec<Meld>, scores: Vec<Score>, score_tai: &ScoreTai) -> Self {
        let tai = scores
            .iter()
            .map(|s| score_tai.get(s).unwrap_or(&0_u8))
            .sum();
        Self { melds, scores, tai }
    }
}

impl std::cmp::Ord for Hu {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tai.cmp(&other.tai)
    }
}

impl std::cmp::Eq for Hu {}

impl std::cmp::PartialEq for Hu {
    fn eq(&self, other: &Self) -> bool {
        self.tai == other.tai
    }
}

impl std::cmp::PartialOrd for Hu {
    fn gt(&self, other: &Self) -> bool {
        self.tai > other.tai
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.tai.cmp(&other.tai))
    }
}

impl std::cmp::PartialEq<Option<Hu>> for Hu {
    fn eq(&self, other: &Option<Hu>) -> bool {
        match other {
            Some(o) => self.eq(o),
            None => false,
        }
    }
}

impl std::cmp::PartialOrd<Option<Hu>> for Hu {
    fn partial_cmp(&self, other: &Option<Hu>) -> Option<std::cmp::Ordering> {
        match other {
            Some(o) => self.partial_cmp(o),
            None => Some(std::cmp::Ordering::Less),
        }
    }
}

fn search_melds(concealed: &ConcealedTiles) -> Vec<Vec<Meld>> {
    let mut poss_melds = Vec::new();
    let mut search: Vec<(ConcealedTiles, Vec<Meld>)> = Vec::from([(concealed.clone(), Vec::new())]);

    while let Some((cur_concealed, cur_melds)) = search.pop() {
        if cur_concealed.is_empty() {
            poss_melds.push(cur_melds);
            continue;
        }

        if cur_melds.is_empty() {
            // Search for eyes
            for (tile, count) in cur_concealed.iter() {
                if count >= &2 {
                    let mut next_hand = cur_concealed.clone();
                    let mut next_melds = cur_melds.clone();
                    next_hand.remove_n(tile, 2).unwrap();
                    next_melds.push(
                        Meld::new(vec![tile.to_owned(); 2], None, crate::meld::MeldType::Eye)
                            .unwrap(),
                    );
                    search.push((next_hand, next_melds));
                }
            }
        } else {
            let (tile, count) = cur_concealed.iter().next().unwrap();
            // Search for pong
            if count >= &3 {
                let mut next_hand = cur_concealed.clone();
                let mut next_melds = cur_melds.clone();
                next_hand.remove_n(tile, 3).unwrap();
                next_melds.push(
                    Meld::new(vec![tile.to_owned(); 3], None, crate::meld::MeldType::Pong).unwrap(),
                );
                search.push((next_hand, next_melds));
            }

            let prev = tile.prev();
            let prev_prev = prev.and_then(|t| t.prev());
            let next = tile.next();
            let next_next = next.and_then(|t| t.next());
            let tiles_to_check = [prev_prev, prev, next, next_next];
            let mut it = tiles_to_check.windows(2);
            while let Some([Some(t1), Some(t2)]) = it.next() {
                if !(cur_concealed.contains_key(t1) && cur_concealed.contains_key(t2)) {
                    continue;
                }
                let mut next_hand = cur_concealed.clone();
                let mut next_melds = cur_melds.clone();
                next_hand.remove_n(t1, 1).unwrap();
                next_hand.remove_n(t2, 1).unwrap();
                next_hand.remove_n(tile, 1).unwrap();
                let chi_tiles = vec![*tile, *t1, *t2];
                next_melds.push(Meld::new(chi_tiles, None, crate::meld::MeldType::Chi).unwrap());
                search.push((next_hand, next_melds));
            }
        }
    }
    poss_melds
}

impl std::cmp::PartialEq<FlowerValue> for Wind {
    fn eq(&self, other: &FlowerValue) -> bool {
        match self {
            Wind::East => matches!(other, FlowerValue::One),
            Wind::South => matches!(other, FlowerValue::Two),
            Wind::West => matches!(other, FlowerValue::Three),
            Wind::North => matches!(other, FlowerValue::Four),
        }
    }
}

pub fn search_hu(
    hand: &Hand,
    discarded_tile: Option<&Tile>,
    scores: Vec<Score>,
    seat_wind: &Wind,
    prevailing_wind: &Wind,
    score_tai: &ScoreTai,
) -> Option<Hu> {
    let mut concealed = hand.concealed().clone();
    if let Some(tile) = discarded_tile {
        concealed.add_n(tile, 1);
    }
    let mut all_scores = scores.clone();
    // [Animal, RedFlower, BlueFlower]
    let mut bonus_tiles = [0_u8; 3];

    for bonus_tile in hand.bonus() {
        match bonus_tile {
            Tile::Animal(_) => {
                all_scores.push(Score::Animal);
                bonus_tiles[0] += 1;
            }
            Tile::Flower(Flower::Red(f)) => {
                if seat_wind == f {
                    all_scores.push(Score::PlayerFlower);
                }
                bonus_tiles[1] += 1;
            }
            Tile::Flower(Flower::Blue(f)) => {
                if seat_wind == f {
                    all_scores.push(Score::PlayerFlower);
                }
                bonus_tiles[2] += 1;
            }
            _ => unreachable!(),
        }
    }

    // CompleteAnimals
    if bonus_tiles[0] == 4 {
        all_scores.push(Score::CompleteAnimals);
    }

    // Complete Flowers
    if bonus_tiles[1] == 4 {
        all_scores.push(Score::CompleteRedFlower);
    }
    if bonus_tiles[2] == 4 {
        all_scores.push(Score::CompleteBlueFlower)
    }

    let mut best_hu = None;

    // TODO check SevenPairs
    if hand.melds().is_empty() {
        // Check ThirteenWonders
        let mut all_pairs = true;
        for (tile, count) in hand.concealed().iter() {
            if count != &2 || count != &4 {
                all_pairs = false;
            }
        }
        if all_pairs {
            let mut scores_with_all_pairs = all_scores.clone();
            scores_with_all_pairs.push(Score::SevenPairs);

            best_hu = Some(Hu::new(
                hand.concealed()
                    .keys()
                    .map(|t| Meld::new(vec![*t; 2], None, MeldType::Eye).unwrap())
                    .collect(),
                scores_with_all_pairs,
                score_tai,
            ))
        }
        // [1Wan, 9Wan, 1Suo, 9Suo, 1Tong, 9Tong, Zhong, Fa, Baiban, East, South, West, North]
    }

    // TODO check ThreeGreatScholars
    // TODO check FourGreatBlessings

    let mut poss_melds = search_melds(&concealed);

    for poss_meld in poss_melds.iter_mut() {
        let mut cur_scores = scores.clone();
        let mut all_melds = hand.melds().clone();
        all_melds.append(poss_meld);

        // [Wan, Suo, Tong, Dragon, Wind]
        let mut suits = [0_u8; 5];
        // [Chi, Pong, Gang, AnGang, Eye]
        let mut meld_types = [0_u8; 5];

        for meld in all_melds.iter() {
            match meld.suit() {
                Suit::Wan => suits[0] += 1,
                Suit::Suo => suits[1] += 1,
                Suit::Tong => suits[2] += 1,
                Suit::Wind => {
                    suits[3] += 1;
                    // Check Wind
                    match meld.tiles().first() {
                        Some(Tile::Wind(w)) => {
                            if w == prevailing_wind {
                                cur_scores.push(Score::PrevailingWind)
                            }
                            if w == seat_wind {
                                cur_scores.push(Score::SeatWind)
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                Suit::Dragon => {
                    suits[4] += 1;
                    cur_scores.push(Score::Dragon);
                }
                _ => unreachable!(),
            }
            match meld.meld_type() {
                MeldType::Chi => meld_types[0] += 1,
                MeldType::Pong => meld_types[1] += 1,
                MeldType::Gang => meld_types[2] += 1,
                MeldType::AnGang => meld_types[3] += 1,
                MeldType::Eye => meld_types[4] += 1,
            }
        }

        // Check suits
        let num_number_suits: u8 = suits[0..4].iter().sum();
        if num_number_suits == 1 {
            match suits[3] > 0 || suits[4] > 0 {
                true => cur_scores.push(Score::HalfFlush),
                false => cur_scores.push(Score::FullFlush),
            }
        }

        if hand.melds().is_empty() {
            cur_scores.push(Score::AllConcealed);
        }

        // Check all Chi
        if meld_types[0] == 4 {
            let mut two_side_wait = false;
            let mut no_scoring_eyes = true;
            if let Some(dt) = discarded_tile {
                for meld in all_melds.iter() {
                    if meld.meld_type().eq(&MeldType::Chi) && meld.tiles().contains(dt) {
                        if let (Some(_), Some(_)) = (dt.prev(), dt.prev().and_then(|t| t.prev())) {
                            two_side_wait = true;
                        }
                        if let (Some(_), Some(_)) = (dt.next(), dt.next().and_then(|t| t.next())) {
                            two_side_wait = true;
                        }
                    } else if meld.meld_type().eq(&MeldType::Eye)
                        && (meld.suit().eq(&Suit::Dragon)
                            || meld
                                .tiles()
                                .first()
                                .unwrap()
                                .eq(&Tile::Wind(prevailing_wind.to_owned()))
                            || meld
                                .tiles()
                                .first()
                                .unwrap()
                                .eq(&Tile::Wind(seat_wind.to_owned())))
                    {
                        no_scoring_eyes = false;
                    }
                }
            }
            if discarded_tile.is_none() || (two_side_wait && no_scoring_eyes) {
                match hand.bonus().len() {
                    0 => cur_scores.push(Score::PingHu),
                    _ => cur_scores.push(Score::AllChi),
                }
            }
        }

        // Check all Pong
        if !hand.melds().is_empty() && meld_types[0] == 0 {
            cur_scores.push(Score::AllPong);
        }

        // Special Hands (Limit)
        // HiddenTreasure
        if hand.melds().is_empty() && meld_types[0] == 0 {
            cur_scores = vec![Score::HiddenTreasure];
        }
        // AllHonours
        if num_number_suits == 0 {
            cur_scores = vec![Score::AllHonours];
        }
        // AllTerminals
        if all_melds.iter().all(|m| {
            !matches!(m.meld_type(), MeldType::Chi)
                && match m.suit() {
                    Suit::Wan | Suit::Suo | Suit::Tong => match m.tiles().first() {
                        Some(Tile::Wan(v)) | Some(Tile::Suo(v)) | Some(Tile::Tong(v)) => {
                            matches!(v, TileValue::One | TileValue::Nine)
                        }
                        Some(_) => false,
                        None => unreachable!(),
                    },
                    _ => false,
                }
        }) {
            cur_scores = vec![Score::AllTerminals];
        }

        // FullFlushPingHu
        if meld_types[0] == 4 && num_number_suits == 1 {
            cur_scores = vec![Score::FullFlushPingHu];
        }

        let hu = Some(Hu::new(all_melds, cur_scores, score_tai));
        if hu > best_hu {
            best_hu = hu;
        }
    }

    best_hu
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub enum Score {
    Dragon,          // ok
    PrevailingWind,  // ok
    SeatWind,        // ok
    AllConcealed,    // ok
    AllChi,          // ok
    PingHu,          // ok
    AllPong,         // ok
    HiddenTreasure,  // ok
    HalfFlush,       // ok
    FullFlush,       // ok
    FullFlushPingHu, // ok
    AllTerminals,    // ok
    HalfTerminals,
    AllHonours, // ok
    ThirteenWonders,
    Animal,             // ok
    CompleteAnimals,    // ok
    PlayerFlower,       // ok
    CompleteRedFlower,  // ok
    CompleteBlueFlower, // ok
    HuaShang,           // ok
    GangShang,          // ok
    HaiDiLao,           // ok
    QiangGang,          // ok
    HuaHu,
    ThreeGreatScholars,
    DaSiXi,
    FourGreatBlessings,
    XiaoSiXi,
    SevenPairs,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        hand::Hand,
        hu::{search_hu, Hu, Score},
        meld::{Meld, MeldType},
        tile::{Animal, Tile, TileValue, Wind},
    };

    #[test]
    fn test_search_hu_pinghu() {
        let mut hand = Hand::new();
        let melds = vec![
            Meld::new(
                vec![Tile::Wan(TileValue::Two), Tile::Wan(TileValue::Three)],
                Some(Tile::Wan(TileValue::One)),
                MeldType::Chi,
            )
            .unwrap(),
            Meld::new(
                vec![Tile::Wan(TileValue::Two), Tile::Wan(TileValue::Three)],
                Some(Tile::Wan(TileValue::One)),
                MeldType::Chi,
            )
            .unwrap(),
            Meld::new(
                vec![Tile::Wan(TileValue::Two), Tile::Wan(TileValue::Three)],
                Some(Tile::Wan(TileValue::One)),
                MeldType::Chi,
            )
            .unwrap(),
        ];
        hand.draw(&Tile::Wan(TileValue::Two));
        hand.draw(&Tile::Wan(TileValue::Two));
        hand.draw(&Tile::Wan(TileValue::Two));
        hand.draw(&Tile::Wan(TileValue::Three));
        hand.draw(&Tile::Wan(TileValue::Three));
        hand.draw(&Tile::Wan(TileValue::Three));
        hand.draw(&Tile::Wan(TileValue::Four));
        hand.draw(&Tile::Wan(TileValue::Five));
        hand.draw(&Tile::Wan(TileValue::Four));
        hand.draw(&Tile::Wan(TileValue::Four));
        for meld in melds.iter() {
            hand.meld(meld.to_owned()).unwrap();
        }

        let found_hu = search_hu(
            &hand,
            Some(&Tile::Wan(TileValue::Three)),
            Vec::new(),
            &Wind::South,
            &Wind::South,
            &HashMap::new(),
        )
        .unwrap();
        assert_eq!(found_hu.melds, melds);
    }

    // #[test]
    fn test_search_hu_allchi() {
        let mut hand = Hand::new();
        let melds = vec![
            Meld::new(
                vec![Tile::Wan(TileValue::Two), Tile::Wan(TileValue::Three)],
                Some(Tile::Wan(TileValue::One)),
                MeldType::Chi,
            )
            .unwrap(),
            Meld::new(
                vec![Tile::Wan(TileValue::Two), Tile::Wan(TileValue::Three)],
                Some(Tile::Wan(TileValue::One)),
                MeldType::Chi,
            )
            .unwrap(),
            Meld::new(
                vec![Tile::Wan(TileValue::Two), Tile::Wan(TileValue::Three)],
                Some(Tile::Wan(TileValue::One)),
                MeldType::Chi,
            )
            .unwrap(),
        ];
        hand.draw(&Tile::Wan(TileValue::Two));
        hand.draw(&Tile::Wan(TileValue::Two));
        hand.draw(&Tile::Wan(TileValue::Two));
        hand.draw(&Tile::Wan(TileValue::Three));
        hand.draw(&Tile::Wan(TileValue::Three));
        hand.draw(&Tile::Wan(TileValue::Three));
        hand.draw(&Tile::Wan(TileValue::Four));
        hand.draw(&Tile::Wan(TileValue::Five));
        hand.draw(&Tile::Wan(TileValue::Four));
        hand.draw(&Tile::Wan(TileValue::Four));
        hand.draw(&Tile::Wind(Wind::East));
        for meld in melds.iter() {
            hand.meld(meld.to_owned()).unwrap();
        }
        assert_eq!(
            search_hu(
                &hand,
                Some(&Tile::Wan(TileValue::Three)),
                Vec::new(),
                &Wind::South,
                &Wind::South,
                &HashMap::new()
            ),
            Some(Hu::new(melds, vec![Score::AllChi], &HashMap::new()))
        );
    }
}
