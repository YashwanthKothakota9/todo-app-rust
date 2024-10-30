use super::super::enums::TaskStatus;
use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_tile: &str) -> Self {
        let base = Base {
            title: input_tile.to_string(),
            status: TaskStatus::DONE,
        };
        Done { super_struct: base }
    }
}
