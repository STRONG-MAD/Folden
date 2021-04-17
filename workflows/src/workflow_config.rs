use std::{fs, io, path::{Path, PathBuf}};

use serde::{Serialize, Deserialize};

use crate::{actions::WorkflowActions, event::WorkflowEvent};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowConfig {
    pub watch_recursive: bool,
    pub apply_on_startup_on_existing_files: bool,
    pub panic_handler_on_error: bool,
    pub event: WorkflowEvent,
    pub actions: Vec<WorkflowActions>
}

impl WorkflowConfig {
    pub fn generate_config(&self, path: &Path) -> io::Result<()> {
        fs::write(path, toml::to_vec(*Box::new(self)).unwrap())
    }
    
    pub fn from_config(path: &PathBuf) -> Self {
        let data = fs::read(path).unwrap();
        Self::from(data)
    }
}

impl From<Vec<u8>> for WorkflowConfig {
    fn from(bytes: Vec<u8>) -> Self {
        toml::from_slice(&bytes).unwrap()
    }
}