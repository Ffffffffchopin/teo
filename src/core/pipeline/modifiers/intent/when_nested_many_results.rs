use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::{Context, Intent};

#[derive(Debug, Clone)]
pub struct WhenNestedManyResultsModifier {
    pipeline: Pipeline
}

impl WhenNestedManyResultsModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return WhenNestedManyResultsModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Modifier for WhenNestedManyResultsModifier {

    fn name(&self) -> &'static str {
        "whenNestedManyResults"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.intent {
            Intent::NestedManyResult(_) => self.pipeline.process(ctx.clone()).await,
            _ => ctx
        }
    }
}