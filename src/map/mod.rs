use std::collections::{HashMap, VecDeque};

use crate::id::SpaceTimeId;

#[derive(Debug)]
struct SpaceTimeIdSet<T> {
    inner: Inner<T>,
}

#[derive(Debug)]
enum Inner<T> {
    Value(T),
    Children(Children<T>),
}

#[derive(Debug)]
struct Children<T> {
    mask: u8,                           // どの子が存在するかをビットマスク
    nodes: Vec<Box<SpaceTimeIdSet<T>>>, // 存在する子だけ格納
}

impl<T: Default> SpaceTimeIdSet<T> {
    fn new() -> Self {
        Self {
            inner: Inner::Children(Children {
                mask: 0,
                nodes: Vec::new(),
            }),
        }
    }

    fn insert(&mut self, id: SpaceTimeId, v: T) -> Result<String> {
        let mut dig = self;

        for i in 0..id.z() {}
    }
}
