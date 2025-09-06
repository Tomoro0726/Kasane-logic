use crate::id::SpaceTimeId;

#[derive(Debug)]
struct SpaceTimeIdMap<T> {
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
    nodes: Vec<Box<SpaceTimeIdMap<T>>>, // 存在する子だけ格納
}

impl<T: Default> SpaceTimeIdMap<T> {
    fn new() -> Self {
        Self {
            inner: Inner::Children(Children {
                mask: 0,
                nodes: Vec::new(),
            }),
        }
    }

    fn insert(&mut self, id: SpaceTimeId, v: T) {
        let mut dig = self;
        for i in 0..id.z() {
            //ここで次のIDの判定を行う
            //次のIDがあるmaskの値を計算する
        }
    }

    //入力されたIDがあるZoomLeveLにおいて、どこに所属するのかを分割して返す関数
    //u8はMaskに対応している
    //自分より以下のサイズのIDがotherされた場合はPanic
    fn split_id(id: SpaceTimeId, top_z: u8) -> Vec<(SpaceTimeId, u8)> {
        if top_z > id.z() {
            panic!("知らん");
        };

        //Xについての境界を考える

        //Yについての境界を考える
        todo!()
    }
}

//境界のデータを作成する
fn intervals_and_values_f(n: u8, start: i32, end: i32) -> Vec<((i32, i32), bool)> {
    let step = 1 << n; // 2^n
    let mut intervals = Vec::new();

    let mut current = start;
    let mut b = (start >> n) << n;
    if b < start {
        b += step;
    }

    while current <= end {
        // saturating_sub でオーバーフロー防止
        let next = b.saturating_sub(1);
        let interval_end = if next > end { end } else { next };
        let value = ((current >> n) & 1) == 0;
        intervals.push(((current, interval_end), value));

        current = b;
        b = b.saturating_add(step); // u32 と i32 両方で安全
    }

    intervals
}

fn intervals_and_values_xy(n: u8, start: u32, end: u32) -> Vec<((u32, u32), bool)> {
    let step = 1 << n; // 2^n
    let mut intervals = Vec::new();

    let mut current = start;
    let mut b = (start >> n) << n;
    if b < start {
        b += step;
    }

    while current <= end {
        let next = b.saturating_sub(1);
        let interval_end = if next > end { end } else { next };
        let value = ((current >> n) & 1) == 0;
        intervals.push(((current, interval_end), value));

        current = b;
        b = b.saturating_add(step);
    }

    intervals
}
