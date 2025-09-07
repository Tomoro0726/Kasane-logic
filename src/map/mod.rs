use crate::id::SpaceTimeId;
use std::collections::hash_map::Iter;

#[derive(Debug)]
pub struct SpaceTimeIdMap<T> {
    //地表面より上のOcTree
    up_inner: Inner<T>,

    //地表面より下のOcTree
    down_inner: Inner<T>,
}

#[derive(Debug)]
pub enum Inner<T> {
    Value(T),
    Children(Children<T>),
}

#[derive(Debug)]
pub struct Children<T> {
    pub mask: u8, // どの子が存在するかをビットマスク
    pub nodes: Vec<Box<Inner<T>>>, // 存在する子だけ格納
                  //ここに時間に関する情報を追加(おそらくu64のInterValSetを実装するのが手っ取り早い)
}

impl<T: Default + Clone> SpaceTimeIdMap<T> {
    pub fn new() -> Self {
        Self {
            up_inner: Inner::Children(Children {
                mask: 0,
                nodes: Vec::new(),
            }),
            down_inner: Inner::Children(Children {
                mask: 0,
                nodes: Vec::new(),
            }),
        }
    }
}

use std::collections::hash_map::{IterMut, Keys, Values, ValuesMut};

trait SpaceTimeIdMapTrait<T>
where
    T: 'static,
{
    type Iter<'a>: Iterator<Item = (&'a SpaceTimeId, &'a T)>
    where
        Self: 'a,
        T: 'a;

    //新しいMapの作成
    fn new() -> Self;

    //時空間IDに対してValueを挿入
    //既存のValueがある場合は上書き
    fn insert(&mut self, id: SpaceTimeId, value: T);

    //時空間IDに対してValueを挿入
    //既存のValueがある場合はエラー
    fn or_insert(&mut self, id: SpaceTimeId, value: T) -> Result<(), String>;

    //時空間ID
    fn get_range<'a>(&'a self, range: &'a SpaceTimeId) -> Self::Iter<'a>;
    fn remove_range(&mut self, range: &SpaceTimeId);
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;

    fn iter<'a>(&'a self) -> Iter<'a, SpaceTimeId, T>;
    fn iter_mut<'a>(&'a mut self) -> IterMut<'a, SpaceTimeId, T>;
    fn keys<'a>(&'a self) -> Keys<'a, SpaceTimeId, T>;
    fn values<'a>(&'a self) -> Values<'a, SpaceTimeId, T>;
    fn values_mut<'a>(&'a mut self) -> ValuesMut<'a, SpaceTimeId, T>;
}
