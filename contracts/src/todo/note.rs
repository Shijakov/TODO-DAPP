use ink::prelude::string::String;

#[derive(scale::Decode, scale::Encode, Clone, Default, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Note {
    pub id: u64,
    pub completed: bool,
    pub title: String,
    pub description: String,
    pub is_repeating: bool,
}

impl Note {
    pub fn new(id: u64, title: String, description: String, is_repeating: bool) -> Self {
        Note {
            id,
            title,
            description,
            completed: false,
            is_repeating,
        }
    }
}