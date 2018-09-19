use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HighestHand {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Eq, Ord)]
struct RankGroup {
    rank: usize,
    count: usize,
}

static SUITS: &[&str] = &["H", "S", "C", "D"];
static RANKS: &[&str] = &["AceLow", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

impl PartialOrd for RankGroup {
    fn partial_cmp(&self, other: &RankGroup) -> Option<Ordering> {
        if self.count == other.count {
            return Some(self.rank.cmp(&other.rank));
        }
        Some(self.count.cmp(&other.count))
    }
}

impl PartialEq for RankGroup {
    fn eq(&self, other: &RankGroup) -> bool {
        self.count == other.count && self.rank == other.rank
    }
}

#[derive(PartialEq, Eq, Ord)]
struct Hand<'a> {
    original: &'a str,
    ranks: Vec<usize>,
    groups: Vec<RankGroup>,
    is_flush: bool,
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let (self_highest, other_highest) = (self.highest_hand(), other.highest_hand());
        if self_highest == other_highest {
            if let (Some(ranks_self), Some(ranks_other)) =
                (self.straight_ranks(), other.straight_ranks())
            {
                // If we have straights, the ace can be played low, so we need to compare the new ranks
                return ranks_self.partial_cmp(&ranks_other);
            }

            return self.groups.partial_cmp(&other.groups);
        }

        Some(self_highest.cmp(&other_highest))
    }
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(hand_string: &'a str) -> Self {
        let cards: Vec<_> = hand_string.split(" ").collect();

        let suits = cards.iter().map(|card| {
            SUITS
                .iter()
                .position(|i| *i == &card[card.len() - 1..])
                .unwrap()
        });

        let ranks = cards.iter().map(|card| {
            RANKS
                .iter()
                .position(|i| *i == &card[..card.len() - 1])
                .unwrap()
        });

        let mut cards: Vec<_> = ranks.zip(suits).collect();
        cards.sort();

        let sorted_ranks = cards.iter().rev().map(|(r, _)| *r).collect();
        let is_flush = cards.iter().all(|(_, s)| *s == cards[0].1);

        let mut groups = HashMap::new();
        for card in cards {
            *groups.entry(card.0).or_insert(0) += 1;
        }

        let mut groups = groups.iter().collect::<Vec<(&usize, &usize)>>();
        groups.sort_by(|&a, &b| {
            if a.1 != b.1 {
                a.1.cmp(b.1).reverse()
            } else {
                a.0.cmp(b.0).reverse()
            }
        });

        let groups = groups
            .iter()
            .map(|(&rank, &count)| RankGroup { rank, count })
            .collect();

        Hand {
            original: hand_string,
            ranks: sorted_ranks,
            groups,
            is_flush,
        }
    }
}

impl<'a> Hand<'a> {
    fn highest_hand(&self) -> HighestHand {
        let is_straight = self.straight_ranks().is_some();

        if self.is_flush && is_straight {
            return HighestHand::StraightFlush;
        }

        if self.groups[0].count == 4 {
            return HighestHand::FourOfAKind;
        }

        if self.groups[0].count == 3 && self.groups[1].count == 2 {
            return HighestHand::FullHouse;
        }

        if self.is_flush {
            return HighestHand::Flush;
        }

        if is_straight {
            return HighestHand::Straight;
        }

        if self.groups[0].count == 3 {
            return HighestHand::ThreeOfAKind;
        }

        if self.groups[0].count == 2 && self.groups[1].count == 2 {
            return HighestHand::TwoPair;
        }

        if self.groups[0].count == 2 {
            return HighestHand::OnePair;
        }

        HighestHand::HighCard
    }

    fn straight_ranks(&self) -> Option<Vec<usize>> {
        let mut ranks = self.ranks.clone();

        if ranks[0] == 13 && ranks[1] == 4 {
            // Ace is played low
            ranks.remove(0);
            ranks.push(0);
        }

        for i in 1..5 {
            if ranks[i] != ranks[i - 1] - 1 {
                return None;
            }
        }

        Some(ranks)
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hands: Vec<Hand> = hands.iter().map(|h| Hand::from(*h)).collect();
    hands.sort();
    hands.reverse();

    Some(
        hands
            .iter()
            .take_while(|h| h.ranks == hands[0].ranks)
            .map(|h| h.original)
            .collect::<Vec<&str>>(),
    )
}
