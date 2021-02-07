use std::{fs, ops::Deref, sync::Arc, thread};

use tokio::sync::{RwLock, RwLockWriteGuard, mpsc};

use crate::mapping::Mapping;
use generated_types::HandlerChannelMessage;
use folder_handler::handlers_json::HandlersJson;
use crate::{config::Config, mapping::HandlerMapping};

pub mod inter_process;

pub struct Server {
    pub config: Arc<Config>,
    pub mapping: Arc<RwLock<Mapping>>,
    pub handlers_json: Arc<HandlersJson>,
}

impl Server {
    pub async fn save_mapping(&self) -> Result<(), std::io::Error> {
        let mapping = self.mapping.read().await;
        let mapping = mapping.deref();
        let mapping_data: Vec<u8> = mapping.into();
        fs::write(&self.config.mapping_state_path, mapping_data)
    }
}

pub fn start_handler_thread(
    mut mapping: RwLockWriteGuard<Mapping>, handlers_json: Arc<HandlersJson>, 
    directory_path: String, handler_type_name: String, handler_config_path: String) {
    match handlers_json.get_handler_by_name(&handler_type_name) {
        Ok(_handler) => {
            let (tx, rx) = mpsc::channel::<HandlerChannelMessage>(2);
            let handler_type_name_clone = handler_type_name.clone();
            thread::spawn(move || {
                let rx = rx;
                let handlers_json = handlers_json;
                let handler = handlers_json.get_handler_by_name(&handler_type_name_clone).unwrap();
                handler.watch(rx);
            });
            
            // Insert or update the value of the current handled directory
            mapping.directory_mapping.insert(directory_path, HandlerMapping {
                handler_thread_tx: Option::Some(tx),
                handler_type_name,
                handler_config_path,
            });
        },
        Err(e) => panic!(e)
    }
}