use logic::{
    id::{
        self,
        DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single},
        SpaceTimeId,
    },
    set::SpaceTimeIdSet,
};

fn main() {
    let id2 = SpaceTimeId::new(2, LimitRange(1, 3), Single(1), LimitRange(2, 1), 0, Any).unwrap();

    for id in id2.to_pure() {
        print!("{},", id);
    }
    println!("");
    // println!("-----------");

    for id in id2.complement().to_pure() {
        print!("{},", id);
    }

    // println!("{}", id1);
}
