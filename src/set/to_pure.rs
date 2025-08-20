use crate::{id::SpaceTimeId, set::SpaceTimeIdSet};

impl SpaceTimeIdSet {
    pub fn to_pure(&self) -> Vec<SpaceTimeId> {
        let mut result = vec![];
        for stid in &self.inner {
            let stid_pure = stid.pure();
            for pure in stid_pure {
                result.push(pure);
            }
        }
        return result;
    }
}
