use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct IsEmailItem {
    regex: Regex
}

impl IsEmailItem {
    pub fn new() -> Self {
        return IsEmailItem {
            regex: Regex::new(r"^\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b$").unwrap()
        };
    }
}

#[async_trait]
impl Item for IsEmailItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            Some(s) => {
                if self.regex.is_match(s) {
                    Ok(ctx)
                } else {
                    Err(ctx.with_invalid("string value is not email"))
                }
            }
            None => {
                Err(ctx.internal_server_error("isEmail: value is not string"))
            }
        }
    }
}
