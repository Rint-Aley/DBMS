use super::Field;

pub enum Filter {
    Equal,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    Contains,
    StartsWith,
    EndsWith,
}

pub struct FilterOption {
    field: Field,
    filter: Filter,
}

impl FilterOption {
    pub fn new(field: Field, filter: Filter) -> Result<Self, String> {
        // TODO: add validation
        Ok(FilterOption { field, filter })
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn filter(&self) -> &Filter {
        &self.filter
    }
}
