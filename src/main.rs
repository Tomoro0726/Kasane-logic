use logic::{
    id::{
        DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single},
        SpaceTimeId,
    },
    set::SpaceTimeIdSet,
};

fn main() {
    let id1 = SpaceTimeId::new(3, LimitRange(3, 4), Any, LimitRange(3, 3), 0, Any).unwrap();

    let set1 = SpaceTimeIdSet::from(id1);

    let id2 = SpaceTimeId::new(
        3,
        LimitRange(0, 4),
        LimitRange(1, 4),
        LimitRange(3, 4),
        3,
        LimitRange(30, 40),
    )
    .unwrap();

    println!("{:?}", id2.complement());

    // println!("{}", id1);
}
