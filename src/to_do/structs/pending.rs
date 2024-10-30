use super::super::enums::TaskStatus;
use super::super::traits::create::Create;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;
use super::base::Base;

pub struct Pending {
    pub super_struct: Base,
}

impl Get for Pending {}
impl Create for Pending {}
impl Edit for Pending {}

impl Pending {
    pub fn new(input_tile: &str) -> Self {
        let base = Base {
            title: input_tile.to_string(),
            status: TaskStatus::PENDING,
        };
        Pending { super_struct: base }
    }
}
