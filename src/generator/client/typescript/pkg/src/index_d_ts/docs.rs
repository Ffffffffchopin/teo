use inflector::Inflector;
use crate::core::action::{Action, FIND_FIRST_HANDLER, FIND_UNIQUE_HANDLER, CREATE_HANDLER, UPDATE_HANDLER, AGGREGATE_HANDLER, ResData, FIND_MANY_HANDLER, UPSERT_HANDLER};
use crate::generator::lib::cases::{field_description, field_localized_name, model_api_object_description, model_localized_name, model_localized_name_word_case, relation_description, relation_localized_name};
use crate::core::field::Field;
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::relation::Relation;

pub(crate) fn simple_model_example(name: &str, model: &Model) -> String {
    if model.has_action(Action::from_u32(FIND_MANY_HANDLER)) {
        let singular_var = model.name().to_camel_case();
        let plural_var = &singular_var.to_plural();
        let plural_word = &plural_var.to_word_case();
        format!(r#" * @example
 * ```ts
 * // Fetch zero or more {plural_word}
 * const {{ {plural_var}: data }} = await {name}.{singular_var}.findMany()
 * ```
 *"#)
    } else {
        "".to_owned()
    }
}

pub(crate) fn main_object_doc(name: &str, graph: &Graph) -> String {
    let pascal_name = name.to_pascal_case();
    let example = if let Some(model) = graph.models().iter().find(|m| { m.has_action(Action::from_u32(FIND_MANY_HANDLER))}) {
        simple_model_example(name, model)
    } else {
        "".to_owned()
    };
    format!(r#"/**
 * ##  {pascal_name} API Client
 *
 * {pascal_name} API client for TypeScript & javaScript. It supports both browser and
 * node.js. It's generated by the fantastic Teo framework.
 *
{example}
 */"#)
}

pub(crate) fn action_group_doc(name: &str, model: &Model) -> String {
    let localized = model_localized_name(model);
    let description = model_api_object_description(model);
    let example = simple_model_example(name, model);
    format!(r#"/**
 * ## {localized}
 *
 * {description}
 *
{example}
 */
"#)
}

pub(crate) fn action_and_model(r#type: Action, model: &Model) -> String {
    let _model_name = model.name();
    let _action_name = r#type.as_handler_str();
    let localized_name = model_localized_name_word_case(model);
    let verb = match r#type.to_u32() {
        FIND_FIRST_HANDLER | FIND_UNIQUE_HANDLER => "find".to_owned(),
        UPSERT_HANDLER => "create or update".to_owned(),
        AGGREGATE_HANDLER => "aggregate on".to_owned(),
        _ => r#type.as_handler_str().to_word_case(),
    };
    let object = match r#type.handler_res_data() {
        ResData::Vec | ResData::Number | ResData::Other => localized_name.to_plural(),
        ResData::Single => {
            match r#type.to_u32() {
                FIND_UNIQUE_HANDLER => format!("a unique {localized_name}"),
                _ => localized_name.articlize()
            }
        },
    };
    format!("{verb} {object}")
}

pub(crate) fn action_doc(name: &str, r#type: Action, model: &Model) -> String {
    let model_name = model.name();
    let model_name_camel_case = model_name.to_camel_case();
    let action_name = r#type.as_handler_str();
    let action_name_camel_case = action_name.to_camel_case();
    let lower_case_main_doc = action_and_model(r#type, model);
    let main_doc = lower_case_main_doc.to_sentence_case();
    format!(r#"/**
 * {main_doc}.
 * @param {{{model_name}{action_name}Args}} args - Arguments to {lower_case_main_doc}.
 * @example
 * // {main_doc}.
 * const result = await {name}.{model_name_camel_case}.{action_name_camel_case}({{
 *     // data to {lower_case_main_doc}
 * }})
 */
"#)
}

pub(crate) fn select_doc(model: &Model) -> String {
    let model_word = model.name().to_word_case();
    format!(r#"/**
 * Select scalar fields to fetch from the {model_word} model.
 */"#)
}

pub(crate) fn include_doc(model: &Model) -> String {
    let model_word = model.name().to_word_case();
    format!(r#"/**
 * Include relations to fetch from the {model_word} model.
 */"#)
}

pub(crate) fn create_or_update_doc(model: &Model, action: Action) -> String {
    let verb_and_object = action_and_model(action, model);
    format!(r#"/**
 * Data needed to {verb_and_object}.
 */"#)
}

pub(crate) fn credentials_doc(model: &Model, action: Action) -> String {
    let verb_and_object = action_and_model(action, model);
    format!(r#"/**
 * Credential data needed to {verb_and_object}.
 */"#)
}

pub(crate) fn unique_connect_doc(model: &Model) -> String {
    let object = model_localized_name_word_case(model);
    format!(r#"/**
 * Find which {object} to connect.
 */"#)
}

pub(crate) fn unique_connect_create_doc(model: &Model) -> String {
    let object = model_localized_name_word_case(model);
    format!(r#"/**
 * If it's not found, what data to pass to create the connected {object}.
 */"#)
}

pub(crate) fn unique_where_doc(model: &Model) -> String {
    let object = model_localized_name_word_case(model);
    format!(r#"/**
 * The unique filter to find the {object}.
 */"#)
}

pub(crate) fn where_doc(model: &Model) -> String {
    let object = model_localized_name_word_case(model).to_plural();
    format!(r#"/**
 * The filter to find {object}.
 */"#)
}

pub(crate) fn where_doc_first(model: &Model) -> String {
    let object = model_localized_name_word_case(model).articlize();
    format!(r#"/**
 * The filter to find {object}.
 */"#)
}

pub(crate) fn order_by_doc(model: &Model) -> String {
    let object = model_localized_name_word_case(model).articlize();
    format!(r#"/**
 * Determine the order of {object} to fetch.
 */"#)
}

pub(crate) fn take_doc(model: &Model) -> String {
    let object = model_localized_name_word_case(model).to_plural();
    format!(r#"/**
 * How many {object} to take. If cursor is set and this value is negative, take from the other direction.
 */"#)
}

pub(crate) fn skip_doc(model: &Model) -> String {
    let object = model_localized_name_word_case(model).to_plural();
    format!(r#"/**
 * Skip the first `n` {object}.
 */"#)
}

pub(crate) fn cursor_doc(model: &Model) -> String {
    let object = model_localized_name_word_case(model).to_plural();
    format!(r#"/**
 * Sets the position for searching for {object}.
 */"#)
}

pub(crate) fn page_size_doc(model: &Model) -> String {
    let object = model_localized_name_word_case(model).to_plural();
    format!(r#"/**
 * Sets the page size for the returned {object} data.
 */"#)
}

pub(crate) fn page_number_doc(model: &Model) -> String {
    let object = model_localized_name_word_case(model).to_plural();
    format!(r#"/**
 * Sets the page number of {object} data.
 */"#)
}

pub(crate) fn field_doc(field: &Field) -> String {
    let name = field_localized_name(field);
    let desc = field_description(field);
    format!(r#"/**
 * **{name}**
 *
 * {desc}
 */"#)
}

pub(crate) fn relation_doc(relation: &Relation) -> String {
    let name = relation_localized_name(relation);
    let desc = relation_description(relation);
    format!(r#"/**
 * **{name}**
 *
 * {desc}
 */"#)
}

pub(crate) fn nested_create_doc(model: &Model, many: bool) -> String {
    let mut object = model_localized_name_word_case(model);
    if many {
        object = object.to_plural();
    } else {
        object = object;
    }
    let article = if many { "" } else { "a " };
    format!(r#"/**
 * Create {article}connected {object}.
 */"#)
}

pub(crate) fn nested_connect_doc(model: &Model, many: bool) -> String {
    let mut object = model_localized_name_word_case(model);
    if many {
        object = object.to_plural();
    } else {
        object = object;
    }
    let article = if many { "" } else { "an " };
    format!(r#"/**
 * Connect to {article}existing {object}.
 */"#)
}

pub(crate) fn nested_create_or_connect_doc(model: &Model, many: bool) -> String {
    let mut object = model_localized_name_word_case(model);
    if many {
        object = object.to_plural();
    } else {
        object = object;
    }
    let article = if many { "" } else { "an " };
    let _article2 = if many { "zero or more " } else { "an " };
    format!(r#"/**
 * Connect to {article}existing {object} if it's exist, otherwise create {article}{object}.
 */"#)
}

pub(crate) fn nested_set_doc(model: &Model, many: bool) -> String {
    let mut object = model_localized_name_word_case(model);
    if many {
        object = object.to_plural();
    } else {
        object = object;
    }
    let article = if many { "" } else { "an " };
    format!(r#"/**
 * Set to {article}existing {object}. This unsets the previous relation.
 */"#)
}

pub(crate) fn nested_update_doc(model: &Model, many: bool) -> String {
    let mut object = model_localized_name_word_case(model);
    if many {
        object = object.to_plural();
    } else {
        object = object;
    }
    let article = if many { "" } else { "an " };
    format!(r#"/**
 * Update {article}existing connected {object}.
 */"#)
}

pub(crate) fn nested_upsert_doc(model: &Model, many: bool) -> String {
    let mut object = model_localized_name_word_case(model);
    if many {
        object = object.to_plural();
    } else {
        object = object;
    }
    let article = if many { "" } else { "an " };
    let _article2 = if many { "zero or more " } else { "an " };
    format!(r#"/**
 * Update {article}existing {object} if it's exist, otherwise create {article}{object}.
 */"#)
}

pub(crate) fn nested_disconnect_doc(model: &Model, many: bool) -> String {
    let mut object = model_localized_name_word_case(model);
    if many {
        object = object.to_plural();
    } else {
        object = object;
    }
    let article = if many { "" } else { "an " };
    format!(r#"/**
 * Disconnect from {article}existing {object}.
 */"#)
}

pub(crate) fn nested_delete_doc(model: &Model, many: bool) -> String {
    let mut object = model_localized_name_word_case(model);
    if many {
        object = object.to_plural();
    } else {
        object = object;
    }
    let article = if many { "" } else { "an " };
    format!(r#"/**
 * Delete {article}existing connected {object}.
 */"#)
}

pub(crate) fn with_token_doc() -> String {
    format!(r#"/**
 * Get a new client altered with `token`. This is useful if you use this on a backend server.
 * @param {{string?}} token - The new identity token.
 */"#)
}
