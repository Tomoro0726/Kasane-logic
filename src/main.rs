use Kasane_Logic::{
    id::{
        self,
        DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single},
        SpaceTimeId,
    },
    set::SpaceTimeIdSet,
};

fn main() {
    let id2 = SpaceTimeId::new(3, Any, LimitRange(1, 3), Any, 3, BeforeUnLimitRange(6)).unwrap();
    println!("{},", id2);
    println!("{},", id2.complement());
}
