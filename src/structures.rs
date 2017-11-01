use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Hash, Debug)]
pub struct Letter {
    pub character: char,
    pub next: Vec<Weight>,
}

#[derive(PartialEq, PartialOrd, Hash, Debug)]
pub struct Weight {
    pub character: char,
    pub weight: i32,
}