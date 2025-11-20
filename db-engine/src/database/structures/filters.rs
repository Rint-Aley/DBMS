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
    field: Field,
    filter: Filter,
}

impl FilterOption {
    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn filter(&self) -> &Filter {
        &self.filter
    }
}
