use kasane_logic::{
    id::{
        self,
        DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single},
        SpaceTimeId,
        relation::relation,
    },
    set::SpaceTimeIdSet,
};

fn main() {
    let id = SpaceTimeId::new(
        3,
        LimitRange(1, 1),
        LimitRange(1, 2),
        LimitRange(1, 3),
        0,
        Any,
    )
    .unwrap();

    let id2 = SpaceTimeId::new(
        3,
        LimitRange(1, 1),
        LimitRange(1, 2),
        LimitRange(1, 6),
        30,
        LimitRange(1, 4),
    )
    .unwrap();

    println!("{}", id2);
    println!("{}", id2.complement());
    //println!("{:?}", relation(id, id2));

    // let mut set = SpaceTimeIdSet::from(id);
    // // let mut set2 = SpaceTimeIdSet::from(id2);

    // set.insert(id2);

    // println!("MAIN {}", set);
}
