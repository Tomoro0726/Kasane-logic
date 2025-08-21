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
        LimitRange(1, 3),
        LimitRange(1, 3),
        LimitRange(1, 3),
        3,
        Single(6),
    )
    .unwrap();

    let id2 = SpaceTimeId::new(
        3,
        LimitRange(1, 3),
        LimitRange(1, 3),
        LimitRange(1, 2),
        0,
        Any,
    )
    .unwrap();

    // println!("{}", id2);
    // println!("{}", id2.complement());
    // println!("{:?}", relation(id, id2));

    let mut set = SpaceTimeIdSet::from(id);

    // set.insert(id);

    println!("{}", !set);
}
