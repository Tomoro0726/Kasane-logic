use logic::{
    id::{
        self,
        DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single},
        SpaceTimeId,
    },
    set::SpaceTimeIdSet,
};

fn main() {
    let id1 = SpaceTimeId::new(3, LimitRange(3, 4), Any, LimitRange(3, 3), 0, Any).unwrap();

    let mut set1 = SpaceTimeIdSet::from(id1);

    let id2 = SpaceTimeId::new(
        3,
        LimitRange(0, 4),
        LimitRange(1, 4),
        LimitRange(3, 4),
        3,
        LimitRange(30, 40),
    )
    .unwrap();

    set1.insert(id2);

    println!("{}", id2);

    println!("{}", set1);

    // println!("{}", id1);
}
