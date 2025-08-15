use logic::{
    id::{
        DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single},
        SpaceTimeId,
    },
    set::SpaceTimeIdSet,
};

fn main() {
    let id1 = SpaceTimeId::new(
        3,
        LimitRange(3, 4),
        LimitRange(3, 4),
        LimitRange(3, 3),
        0,
        Any,
    )
    .unwrap();

    let set1 = SpaceTimeIdSet::from(id1);

    println!("{}", set1);

    let id2 = SpaceTimeId::new(
        3,
        LimitRange(3, 4),
        LimitRange(3, 4),
        LimitRange(3, 4),
        3,
        LimitRange(30, 40),
    )
    .unwrap();

    println!("{:?}", id1.containment_relation(&id2));
}
