use super::super::enums::TaskStatus;
use super::base::Base;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_tile: &str) -> Self {
        let base = Base {
            title: input_tile.to_string(),
            status: TaskStatus::PENDING,
        };
        Pending { super_struct: base }
    }
}
