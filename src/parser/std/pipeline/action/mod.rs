use std::sync::Arc;
use crate::core::action::Action;
use crate::core::pipeline::item::Item;
use crate::core::items::action::redirect::RedirectItem;
use crate::core::items::action::when::WhenItem;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn when(args: Vec<Argument>) -> Arc<dyn Item> {
    let pipeline = args.get(1).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap();
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    match value {
        Value::RawOptionChoice(action_value) => {
            Arc::new(WhenItem::new(vec![Action::from_u32(*action_value)], pipeline.clone()))
        }
        Value::RawEnumChoice(enum_member, _) => {
            let action = Action::from_name(enum_member);
            Arc::new(WhenItem::new(vec![action], pipeline.clone()))
        }
        _ => {
            panic!()
        }
    }
}

pub(crate) fn redirect(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    match value {
        Value::RawOptionChoice(action_value) => {
            Arc::new(RedirectItem::new(Action::from_u32(*action_value)))
        }
        Value::RawEnumChoice(enum_member, _) => {
            let action = Action::from_name(enum_member);
            Arc::new(RedirectItem::new(action))
        }
        _ => {
            panic!()
        }
    }
}
