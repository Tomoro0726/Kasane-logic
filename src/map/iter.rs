use crate::{
    id::{DimensionRange, SpaceTimeId},
    map::{Children, Inner, SpaceTimeIdMap},
};

impl<T: Default + Clone> SpaceTimeIdMap<T> {
    pub fn iter(&self) -> impl Iterator<Item = (SpaceTimeId, T)> {
        let mut items = Vec::new();

        // up方向を探索
        self.collect_recursive(&self.up_inner, &mut items, 1);

        // down方向を探索
        self.collect_recursive(&self.down_inner, &mut items, 1);

        items.into_iter()
    }

    fn collect_recursive(&self, inner: &Inner<T>, acc: &mut Vec<(SpaceTimeId, T)>, top_z: u8) {
        match inner {
            Inner::Value(v) => {
                let id = SpaceTimeId::new(top_z, f, x, y, 0, DimensionRange::Any).unwrap();

                acc.push((id, v.clone()));
            }
            Inner::Children(children) => for (i, child) in children.nodes.iter().enumerate() {},
        }
    }
}
