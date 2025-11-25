use crate::database::structures::{self as db, Field};
use crate::grpc_server::table_api::{self as proto, value};
use proto::value::Kind as val;

impl From<proto::Type> for db::Type {
    fn from(value: proto::Type) -> Self {
        match value {
            proto::Type::Bool => db::Type::Boolean(false),
            proto::Type::I8 => db::Type::I8(0),
            proto::Type::I16 => db::Type::I16(0),
            proto::Type::I32 => db::Type::I32(0),
            proto::Type::I64 => db::Type::I64(0),
            proto::Type::U8 => db::Type::U8(0),
            proto::Type::U16 => db::Type::U16(0),
            proto::Type::U32 => db::Type::U32(0),
            proto::Type::U64 => db::Type::U64(0),
            proto::Type::F32 => unimplemented!(),
            proto::Type::F64 => unimplemented!(),
            proto::Type::String => unimplemented!(),
        }
    }
}

impl TryFrom<proto::Value> for db::Type {
    type Error = String;
    fn try_from(value: proto::Value) -> Result<Self, Self::Error> {
        let value = match value.kind {
            Some(value) => value,
            None => {
                return Err(String::from("'Value' doesn't contain a value."));
            }
        };
        Ok(match value {
            val::Bool(data) => db::Type::Boolean(data),
            val::I8(data) => db::Type::I8(data as i8),
            val::I16(data) => db::Type::I16(data as i16),
            val::I32(data) => db::Type::I32(data),
            val::I64(data) => db::Type::I64(data),
            val::U8(data) => db::Type::U8(data as u8),
            val::U16(data) => db::Type::U16(data as u16),
            val::U32(data) => db::Type::U32(data),
            val::U64(data) => db::Type::U64(data),
            val::F32(_) => unimplemented!(),
            val::F64(_) => unimplemented!(),
        })
    }
}

impl From<db::Type> for proto::Value {
    fn from(value: db::Type) -> Self {
        let kind = match value {
            db::Type::Boolean(data) => val::Bool(data),
            db::Type::I8(data) => val::I8(data as i32),
            db::Type::I16(data) => val::I16(data as i32),
            db::Type::I32(data) => val::I32(data),
            db::Type::I64(data) => val::I64(data),
            db::Type::U8(data) => val::U8(data as u32),
            db::Type::U16(data) => val::U16(data as u32),
            db::Type::U32(data) => val::U32(data),
            db::Type::U64(data) => val::U64(data),
            _ => unimplemented!(),
        };
        Self { kind: Some(kind) }
    }
}

impl From<proto::Field> for db::Field {
    fn from(value: proto::Field) -> Self {
        let type_ = value.r#type().into();
        let name = value.name;
        db::Field {
            name,
            type_,
            nullable: false,
        }
    }
}

impl From<Vec<db::Type>> for proto::ValueSequence {
    fn from(value: Vec<db::Type>) -> Self {
        let sequence = value.into_iter().map(|value| value.into()).collect();
        Self { sequence }
    }
}

impl From<Vec<Vec<db::Type>>> for proto::RecordsInfo {
    fn from(value: Vec<Vec<db::Type>>) -> Self {
        let records = value.into_iter().map(|record| record.into()).collect();
        proto::RecordsInfo { records }
    }
}

impl From<proto::Filter> for db::Filter {
    fn from(value: proto::Filter) -> Self {
        match value {
            proto::Filter::Equal => db::Filter::Equal,
            proto::Filter::Less => db::Filter::Less,
            proto::Filter::Greater => db::Filter::Greater,
            proto::Filter::LessEq => db::Filter::LessEq,
            proto::Filter::GreaterEq => db::Filter::GreaterEq,
            proto::Filter::Contains => db::Filter::Contains,
            proto::Filter::StartsWith => db::Filter::StartsWith,
            proto::Filter::EndsWith => db::Filter::EndsWith,
        }
    }
}

impl TryFrom<proto::FilterOption> for db::FilterOption {
    type Error = String;
    fn try_from(value: proto::FilterOption) -> Result<Self, Self::Error> {
        let filter = value.filter().into();
        let field = match value.column {
            Some(value) => value,
            None => {
                return Err(String::from("Field is not specified."));
            }
        }
        .into();
        db::FilterOption::new(field, filter)
    }
}
