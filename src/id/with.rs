use crate::id::{DimensionRange, SpaceTimeId};

impl SpaceTimeId {
    /// z を変更して検証
    pub fn with_z(&self, z: u16) -> Result<Self, String> {
        Self::new(z, self.f, self.x, self.y, self.i, self.t)
    }

    /// f を変更して検証
    pub fn with_f(&self, f: DimensionRange<i64>) -> Result<Self, String> {
        Self::new(self.z, f, self.x, self.y, self.i, self.t)
    }

    /// x を変更して検証
    pub fn with_x(&self, x: DimensionRange<u64>) -> Result<Self, String> {
        Self::new(self.z, self.f, x, self.y, self.i, self.t)
    }

    /// y を変更して検証
    pub fn with_y(&self, y: DimensionRange<u64>) -> Result<Self, String> {
        Self::new(self.z, self.f, self.x, y, self.i, self.t)
    }

    /// i を変更して検証
    pub fn with_i(&self, i: u32) -> Result<Self, String> {
        Self::new(self.z, self.f, self.x, self.y, i, self.t)
    }

    /// t を変更して検証
    pub fn with_t(&self, t: DimensionRange<u32>) -> Result<Self, String> {
        Self::new(self.z, self.f, self.x, self.y, self.i, t)
    }
}
