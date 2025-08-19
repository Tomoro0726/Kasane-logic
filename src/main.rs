use logic::{
    id::{
        self,
        DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single},
        SpaceTimeId,
    },
    set::SpaceTimeIdSet,
};

fn main() {
    let id2 = SpaceTimeId::new(
        3,
        LimitRange(0, 4),
        LimitRange(0, 4),
        LimitRange(3, 4),
        0,
        Any,
    )
    .unwrap();

    for id in id2.to_pure() {
        print!("{},", id);
    }
    println!("-----------");

    for id in id2.complement().to_pure() {
        print!("{},", id);
    }

    // println!("{}", id1);
}
