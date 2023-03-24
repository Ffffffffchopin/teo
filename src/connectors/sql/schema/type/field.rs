use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::database::r#type::DatabaseType;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};

pub trait ToDatabaseType {
    fn to_database_type(&self, dialect: SQLDialect) -> DatabaseType;
}

impl ToDatabaseType for FieldType {
    fn to_database_type(&self, dialect: SQLDialect) -> DatabaseType {
        match dialect {
            SQLDialect::SQLite => default_database_type_sqlite(self),
            SQLDialect::MySQL => default_database_type_mysql(self),
            SQLDialect::PostgreSQL => default_database_type_postgresql(self),
            SQLDialect::MSSQL => default_database_type_mssql(self),
        }
    }
}

fn default_database_type_mssql(field_type: &FieldType) -> DatabaseType {
    match field_type {
        _ => panic!("Unhandled."),
    }
}

fn default_database_type_mysql(field_type: &FieldType) -> DatabaseType {
    match field_type {
        FieldType::Bool => DatabaseType::TinyInt { m: Some(1), u: false },
        FieldType::I32 => DatabaseType::Int { m: None, u: false },
        FieldType::I64 => DatabaseType::BigInt { m: None, u: false },
        FieldType::F32 => DatabaseType::Float { m: None, d: None },
        FieldType::F64 => DatabaseType::Double { m: None, d: None },
        FieldType::String => DatabaseType::VarChar { m: 191, n: None, c: None },
        FieldType::Date => DatabaseType::Date,
        FieldType::DateTime => DatabaseType::DateTime(3),
        FieldType::Enum(enum_def) => DatabaseType::Enum(enum_def.into()),
        FieldType::Decimal => DatabaseType::Decimal { m: Some(65), d: Some(30) },
        FieldType::Vec(_) => panic!(),
        FieldType::HashMap(_) => panic!(),
        FieldType::BTreeMap(_) => panic!(),
        FieldType::Object(_) => panic!(),
        _ => panic!(),
    }
}

fn default_database_type_postgresql(field_type: &FieldType) -> DatabaseType {
    match field_type {
        FieldType::Bool => DatabaseType::Bool,
        FieldType::I32 => DatabaseType::Int { m: None, u: false },
        FieldType::I64 => DatabaseType::BigInt { m: None, u: false },
        FieldType::F32 => DatabaseType::Real,
        FieldType::F64 => DatabaseType::Double { m: None, d: None },
        FieldType::String => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Date => DatabaseType::Date,
        FieldType::DateTime => DatabaseType::Timestamp { p: 3, z: false },
        FieldType::Decimal => DatabaseType::Decimal { m: Some(65), d: Some(30) },
        FieldType::Enum(_) => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Vec(inner) => DatabaseType::Vec(Box::new(default_database_type_postgresql(inner.field_type()))),
        FieldType::HashMap(_) => panic!(),
        FieldType::BTreeMap(_) => panic!(),
        FieldType::Object(_) => panic!(),
        _ => panic!(),
    }
}

fn default_database_type_sqlite(field_type: &FieldType) -> DatabaseType {
    match field_type {
        FieldType::Bool => DatabaseType::Int { m: None, u: false, },
        FieldType::I32 => DatabaseType::Int { m: None, u: false },
        FieldType::I64 => DatabaseType::Int { m: None, u: false },
        FieldType::F32 => DatabaseType::Real,
        FieldType::F64 => DatabaseType::Real,
        FieldType::String => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Date => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::DateTime => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Decimal => DatabaseType::Decimal { m: None, d: None },
        FieldType::Enum(_) => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Vec(_) => panic!(),
        FieldType::HashMap(_) => panic!(),
        FieldType::BTreeMap(_) => panic!(),
        FieldType::Object(_) => panic!(),
        _ => panic!(),
    }
}
