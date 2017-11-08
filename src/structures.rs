#[derive(PartialEq, PartialOrd, Hash, Debug, Clone)]
pub struct Weight {
    pub character: char,
    pub weight: i32,
}

#[derive(PartialEq, PartialOrd, Hash, Debug, Clone)]
pub struct WeightedRandomLetter {
    pub character: char,
    pub start: i32,
    pub end: i32
}