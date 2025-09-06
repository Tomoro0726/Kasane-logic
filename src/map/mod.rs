pub mod insert;
pub mod iter;

#[derive(Debug)]
pub struct SpaceTimeIdMap<T> {
    up_inner: Inner<T>,
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
    pub nodes: Vec<Box<SpaceTimeIdMap<T>>>, // 存在する子だけ格納
                  //ここに時間に関する情報を追加
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
