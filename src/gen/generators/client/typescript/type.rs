use crate::core::field::r#type::{FieldType, FieldTypeOwner};

pub(crate) trait ToTypeScriptType {
    fn to_typescript_type(&self, optional: bool) -> String;
    fn to_typescript_filter_type(&self, optional: bool, server_mode: bool) -> String;
    fn to_typescript_create_input_type(&self, optional: bool, server_mode: bool) -> String;
    fn to_typescript_update_input_type(&self, optional: bool, server_mode: bool) -> String;
    fn to_typescript_update_operation_input(&self, optional: bool, server_mode: bool) -> String;
    fn to_typescript_scalar_update_input_type(&self, optional: bool) -> String;
}

impl ToTypeScriptType for FieldType {
    fn to_typescript_type(&self, optional: bool) -> String {
        let base: String = match self {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => "string".to_string(),
            FieldType::String | FieldType::Date => "string".to_string(),
            FieldType::DateTime => "Date".to_string(),
            FieldType::Bool => "boolean".to_string(),
            FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 => "number".to_string(),
            FieldType::Decimal => "Decimal".to_string(),
            FieldType::Enum(enum_def) => enum_def.name().to_string(),
            FieldType::Vec(internal) => internal.field_type().to_typescript_type(internal.optionality.is_optional()) + "[]",
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(name) => name.to_string(),
        };
        if optional {
            base + " | undefined"
        } else {
            base
        }
    }

    fn to_typescript_filter_type(&self, optional: bool, server_mode: bool) -> String {
        let mut with_generic = false;
        let base: String = match self {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => "string | ObjectId".to_string(),
            FieldType::String => "string | String".to_string(),
            FieldType::Date => "string | Date".to_string(),
            FieldType::DateTime => "string | Date | DateTime".to_string(),
            FieldType::Bool => "boolean | Bool".to_string(),
            FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 => "number | Number".to_string(),
            FieldType::Decimal => if server_mode { "Decimal | Decimal" } else { "string | Decimal | Decimal" }.to_string(),
            FieldType::Enum(enum_def) => {
                let name = enum_def.name();
                with_generic = true;
                if optional {
                    format!(r#"{name} | EnumNullableFilter<{name}> | null"#)
                } else {
                    format!(r#"{name} | EnumFilter<{name}>"#)
                }
            },
            FieldType::Vec(internal) => {
                with_generic = true;
                let create_type = internal.field_type().to_typescript_create_input_type(false, server_mode);
                if optional {
                    format!("{create_type}[] | ArrayNullableFilter<{create_type}> | null")
                } else {
                    format!("{create_type}[] | ArrayFilter<{create_type}>")
                }
            },
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(_name) => "undefined | Unimplemented".to_string(),
        };
        if !with_generic {
            if optional {
                base + "NullableFilter | null"
            } else {
                base + "Filter"
            }
        } else {
            base
        }
    }

    fn to_typescript_create_input_type(&self, optional: bool, server_mode: bool) -> String {
        let base: String = match self {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => "string".to_string(),
            FieldType::String => "string".to_string(),
            FieldType::Decimal => if server_mode { "Decimal" } else { "string | Decimal" }.to_string(),
            FieldType::Date | FieldType::DateTime => "string".to_string(),
            FieldType::Bool => "boolean".to_string(),
            FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 => "number".to_string(),
            FieldType::Enum(enum_def) => enum_def.name().to_string(),
            FieldType::Vec(internal) => internal.field_type().to_typescript_type(internal.optionality.is_optional()) + "[]",
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(name) => name.to_string(),
        };
        if optional {
            base + " | null"
        } else {
            base
        }
    }

    fn to_typescript_update_input_type(&self, optional: bool, server_mode: bool) -> String {
        let update_operation = self.to_typescript_update_operation_input(optional, server_mode);
        let create_input = self.to_typescript_create_input_type(optional, server_mode);
        return format!("{update_operation} | {create_input}");
    }

    fn to_typescript_scalar_update_input_type(&self, optional: bool) -> String {
        let create_input = self.to_typescript_create_input_type(optional, true);
        return format!("{create_input}");
    }

    fn to_typescript_update_operation_input(&self, optional: bool, server_mode: bool) -> String {
        let mut generic = "".to_owned();
        let base: &str = match self {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => "ObjectId",
            FieldType::String => "String",
            FieldType::Date => "Date",
            FieldType::DateTime => "DateTime",
            FieldType::Decimal => "Decimal",
            FieldType::Bool => "Bool",
            FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 => "Number",
            FieldType::Enum(enum_def) => {
                let name = enum_def.name();
                generic = format!("<{name}>");
                "Enum"
            },
            FieldType::Vec(inner) => {
                let create_type = inner.field_type().to_typescript_create_input_type(false, server_mode);
                generic = format!("<{create_type}>");
                "Array"
            },
            _ => panic!(),
        };
        let suffix = "FieldUpdateOperationsInput";
        let prefix = if optional { "Nullable" } else { "" };
        format!("{prefix}{base}{suffix}{generic}")
    }
}
