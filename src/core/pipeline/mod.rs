pub mod ctx;

use std::sync::Arc;
use crate::core::result::Result;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::{Error, Value};

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub items: Vec<Arc<dyn Item>>
}

impl Pipeline {

    pub(crate) fn new() -> Self {
        Self { items: vec![] }
    }

    pub(crate) fn has_any_items(&self) -> bool {
        self.items.len() > 0
    }

    pub(crate) async fn process(&self, ctx: Ctx<'_>) -> Result<Value> {
        let mut ctx = ctx;
        for item in &self.items {
            ctx = item.call(ctx.clone()).await?;
        }
        Ok(ctx.value)
    }

    pub(crate) async fn process_with_ctx_result<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let mut ctx = ctx;
        for item in &self.items {
            ctx = item.call(ctx.clone()).await?;
        }
        Ok(ctx)
    }

    pub(crate) async fn process_into_permission_result(&self, ctx: Ctx<'_>) -> Result<()> {
        let path = ctx.path.clone();
        match self.process(ctx).await {
            Ok(_) => Ok(()),
            Err(error) => if error.is_server_error() {
                Err(error)
            } else {
                Err(Error::permission_error(path, "permission denied"))
            }
        }
    }
}

unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}

impl PartialEq for Pipeline {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
