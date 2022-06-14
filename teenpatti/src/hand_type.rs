use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum HandType {
    // init field if required
    Trail, //3 of same rank
    PureSequence,
    Sequence,
    Flush, //3 cards of the same color,
    Pair,  //2 cards of the same rank
    HighCard,
}

impl From<HandType> for i32 {
    fn from(val: HandType) -> Self {
        match val {
            HandType::Trail => 1,
            HandType::PureSequence => 2,
            HandType::Sequence => 3,
            HandType::Flush => 4,
            HandType::Pair => 5,
            HandType::HighCard => 6,
        }
    }
}