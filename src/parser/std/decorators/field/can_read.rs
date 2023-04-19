use crate::core::field::field::Field;
use crate::parser::ast::argument::Argument;

pub(crate) fn can_read_decorator(args: &Vec<Argument>, field: &mut Field) {
    field.can_read_pipeline = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap().clone()
}
