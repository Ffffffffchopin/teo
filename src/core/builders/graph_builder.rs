use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::action::action::ActionType;
use crate::core::argument::{Argument, FnArgument};
use crate::core::argument::Argument::{PipelineArgument, ValueArgument};
use crate::core::connector::{ConnectorBuilder};
use crate::core::field::*;
use crate::core::pipeline::Pipeline;
use crate::core::value::Value;
use crate::core::builders::model_builder::ModelBuilder;


pub struct GraphBuilder {
    pub(crate) enums: HashMap<&'static str, Vec<&'static str>>,
    pub(crate) models: Vec<ModelBuilder>,
    pub(crate) connector_builder: Option<Box<dyn ConnectorBuilder>>,
    pub(crate) reset_database: bool,
}

impl GraphBuilder {

    pub(crate) fn new() -> GraphBuilder {
        GraphBuilder {
            enums: HashMap::new(),
            models: Vec::new(),
            connector_builder: None,
            reset_database: false
        }
    }

    pub(crate) fn connector_builder(&self) -> &Box<dyn ConnectorBuilder> {
        match &self.connector_builder {
            Some(connector_builder) => connector_builder,
            None => panic!("Graph doesn't have a database connector.")
        }
    }

    pub fn r#enum(&mut self, name: &'static str, values: Vec<&'static str>) {
        self.enums.insert(name, values);
    }

    pub fn model<F: Fn(&mut ModelBuilder)>(&mut self, name: &'static str, build: F) {
        let mut model: ModelBuilder = ModelBuilder::new(name);
        build(&mut model);
        self.models.push(model);
    }

    pub fn reset_database(&mut self) {
        self.reset_database = true;
    }
}