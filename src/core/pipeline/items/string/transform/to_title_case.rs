use async_trait::async_trait;

use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;
use inflector::cases::titlecase::to_title_case;

#[derive(Debug, Copy, Clone)]
pub struct ToTitleCaseItem {}

impl ToTitleCaseItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for ToTitleCaseItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.get_value() {
            Value::String(ref s) => Ok(ctx.with_value(Value::String(to_title_case(s)))),
            _ => Err(ctx.internal_server_error("value is not string"))
        }
    }
}