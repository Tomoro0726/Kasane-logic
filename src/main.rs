use kasane_logic::{
    id::{
        self,
        DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single},
        SpaceTimeId,
    },
    set::SpaceTimeIdSet,
};

fn main() {
    let id = SpaceTimeId::new(3, LimitRange(1, 3), LimitRange(1, 3), Any, 3, Single(6)).unwrap();
    //println!("{},", id);

    let mut set = SpaceTimeIdSet::from(id);

    let id2 = SpaceTimeId::new(3, LimitRange(1, 3), Any, LimitRange(1, 3), 0, Any).unwrap();
    //println!("{},", id2);

    set.insert(id2);

    //println!("{}", set);

    id.relation(other)
}
