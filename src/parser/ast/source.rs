use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::PathBuf;
use to_mut::ToMut;
use to_mut_proc_macro::ToMut;
use crate::parser::ast::action::ActionGroupDeclaration;
use crate::parser::ast::client::ASTClient;
use crate::parser::ast::config::ASTServer;
use crate::parser::ast::connector::ASTConnector;
use crate::parser::ast::constant::Constant;
use crate::parser::ast::data_set::DataSet;
use crate::parser::ast::debug_conf::ASTDebugConf;
use crate::parser::ast::generator::ASTEntity;
use crate::parser::ast::import::ASTImport;
use crate::parser::ast::interface::InterfaceDeclaration;
use crate::parser::ast::middleware::MiddlewareDeclaration;
use crate::parser::ast::model::ASTModel;
use crate::parser::ast::r#enum::ASTEnum;
use crate::parser::ast::static_files::StaticFiles;
use crate::parser::ast::test_conf::ASTTestConf;
use crate::parser::ast::top::Top;

#[derive(ToMut)]
pub(crate) struct Source {
    pub(crate) id: usize,
    pub(crate) path: PathBuf,
    pub(crate) tops: BTreeMap<usize, Top>,
    pub(crate) imports: BTreeSet<usize>,
    pub(crate) constants: BTreeSet<usize>,
    pub(crate) enums: BTreeSet<usize>,
    pub(crate) models: BTreeSet<usize>,
    pub(crate) resolved: bool,
}

pub(crate) struct SourceImportIter<'a> {
    source: &'a Source,
    index: usize,
}

impl<'a> Iterator for SourceImportIter<'a> {
    type Item = &'a ASTImport;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.imports.iter().skip(self.index).next() {
            Some(index) => {
                let top = self.source.tops.get(index).unwrap();
                self.index += 1;
                Some(top.as_import().unwrap())
            }
            None => None,
        }

    }
}

impl Source {

    pub(crate) fn new(source_id: usize, path: PathBuf, tops: BTreeMap<usize, Top>, imports: BTreeSet<usize>, constants: BTreeSet<usize>, enums: BTreeSet<usize>, models: BTreeSet<usize>) -> Self {
        Self {
            id: source_id,
            path,
            tops,
            imports,
            constants,
            enums,
            models,
            resolved: false,
        }
    }

    pub(crate) fn imports(&self) -> SourceImportIter {
        SourceImportIter { source: self, index: 0 }
    }

    pub(crate) fn get_import(&self, id: usize) -> &ASTImport {
        self.tops.get(&id).unwrap().as_import().unwrap()
    }

    pub(crate) fn get_constant(&self, id: usize) -> &Constant {
        self.tops.get(&id).unwrap().as_constant().unwrap()
    }

    pub(crate) fn get_enum(&self, id: usize) -> &ASTEnum {
        self.tops.get(&id).unwrap().as_enum().unwrap()
    }

    pub(crate) fn get_model(&self, id: usize) -> &ASTModel {
        self.tops.get(&id).unwrap().as_model().unwrap()
    }

    pub(crate) fn get_connector(&self, id: usize) -> &ASTConnector {
        self.tops.get(&id).unwrap().as_connector().unwrap()
    }

    pub(crate) fn get_server(&self, id: usize) -> &ASTServer {
        self.tops.get(&id).unwrap().as_server_config().unwrap()
    }

    pub(crate) fn get_entity(&self, id: usize) -> &ASTEntity {
        self.tops.get(&id).unwrap().as_generator().unwrap()
    }

    pub(crate) fn get_client(&self, id: usize) -> &ASTClient {
        self.tops.get(&id).unwrap().as_client().unwrap()
    }

    pub(crate) fn get_data_set(&self, id: usize) -> &DataSet {
        self.tops.get(&id).unwrap().as_data_set().unwrap()
    }

    pub(crate) fn get_debug_conf(&self, id: usize) -> &ASTDebugConf {
        self.tops.get(&id).unwrap().as_debug_conf().unwrap()
    }

    pub(crate) fn get_test_conf(&self, id: usize) -> &ASTTestConf {
        self.tops.get(&id).unwrap().as_test_conf().unwrap()
    }

    pub(crate) fn get_middleware(&self, id: usize) -> &MiddlewareDeclaration {
        self.tops.get(&id).unwrap().as_middleware().unwrap()
    }

    pub(crate) fn get_action_group(&self, id: usize) -> &ActionGroupDeclaration {
        self.tops.get(&id).unwrap().as_action_group().unwrap()
    }

    pub(crate) fn get_interface(&self, id: usize) -> &InterfaceDeclaration {
        self.tops.get(&id).unwrap().as_interface().unwrap()
    }

    pub(crate) fn get_static_files(&self, id: usize) -> &StaticFiles {
        self.tops.get(&id).unwrap().as_static_files().unwrap()
    }
}

impl fmt::Debug for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source(\"{}\")", self.path.to_str().unwrap())
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}
