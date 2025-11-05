use super::Field;

pub enum Filter {
    Equal,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    Contains,
    StratsWith,
    EndsWith,
}

pub struct FilterOption {
    filed: Field,
    filter: Filter,
}
