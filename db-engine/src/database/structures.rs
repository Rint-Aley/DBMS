pub mod dbtype;
pub mod field;
pub mod filters;
pub mod free_space;
pub mod table_metadata;

use bincode::Decode;
use bincode::Encode;
pub use dbtype::Type;
pub use field::Field;
pub use filters::Filter;
pub use filters::FilterOption;
pub use free_space::FreeSpace;
pub use table_metadata::TableMetadata;

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Hash)]
pub struct DataPosition {
    pub page: u64,
    pub cell: u16,
}
