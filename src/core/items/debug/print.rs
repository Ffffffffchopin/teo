use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct PrintItem {
    label: Option<Value>
}

impl PrintItem {
    pub fn new(label: Option<Value>) -> Self {
        Self {
            label
        }
    }
}

#[async_trait]
impl Item for PrintItem {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        if let Some(label) = &self.label {
            let label_resolved = label.resolve(ctx.clone()).await?;
            match label_resolved.as_str() {
                Some(label_str) => println!("{}: {:?}", label_str, ctx.value),
                None => Err(ctx.internal_server_error("print: label is not string"))?
            }
        } else {
            println!("{:?}", ctx.value);
        }
        Ok(ctx)
    }
}
