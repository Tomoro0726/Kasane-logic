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
        LimitRange(3, 4),
        3,
        LimitRange(0, 30),
    )
    .unwrap();

    println!("{}", id1.change_scale(Some(9), Some(1)).unwrap());
}

// let id1 = SpaceTimeId::new(
//         3,
//         LimitRange(3, 4),
//         LimitRange(3, 4),
//         LimitRange(3, 4),
//         3,
//         LimitRange(0, 30),
//     )
//     .unwrap();

//     let set1 = SpaceTimeIdSet::from(id1);

//     println!("{}", set1);

//     let id2 = SpaceTimeId::new(
//         3,
//         LimitRange(3, 4),
//         LimitRange(3, 4),
//         LimitRange(3, 4),
//         0,
//         Any,
//     )
//     .unwrap();

//     println!("{:?}", id1.containment_relation(&id2));
