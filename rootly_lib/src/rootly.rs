use crate::model::prelude::*;
use crate::model::ReadableModel;
use crate::parsers;
use crate::storage_api::Storage;
use config::{Config, Environment, File};
use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use std::path::{Path, MAIN_SEPARATOR_STR};
use std::string::ToString;

const DEFAULT_DATA_PATH: &str = "rootly_data";
const ROOTLY_CONFIG_FILE_NAME: &str = "rootly.ctx";
const DEFAULT_LOG_PATH: &str = "log";
const DEFAULT_TRACE_PATH: &str = "trace";


pub struct Rootly {
    config: Config,
    storage: Box<dyn Storage>,
    ini: IniFile,
}

impl Rootly {
    fn new(data_path: Option<&str>, storage: Box<dyn Storage>) -> Rootly {
        let mut config_builder = Config::builder().
            add_source(Environment::with_prefix("ROOTLY"));
        let data_path_obj:&Path;
        if data_path.is_some() {
            let data_path_str = data_path.unwrap();
            data_path_obj = Path::new(data_path_str);
            let config_path = data_path_obj.join("rootly.toml");
            config_builder = config_builder.add_source(File::with_name(config_path.to_str().unwrap()));
            config_builder = config_builder.set_default("data_path", data_path.clone()).unwrap();
        }
        let config = config_builder.build().unwrap();
        let mut rootly_ini = None;
        if storage.is_file_exists(ROOTLY_CONFIG_FILE_NAME) {
            rootly_ini = storage.read(ROOTLY_CONFIG_FILE_NAME, None, None);
        }
        let ini:IniFile;
        if rootly_ini.is_none(){
            ini = IniFile::build_default();
            storage.write(ROOTLY_CONFIG_FILE_NAME, ini.to_bytes(), None);
        }
        else {
            let rootly_ini = rootly_ini.unwrap();
            ini = IniFile::from_bytes(&rootly_ini);
        }
        println!("{:?}", ini);
        Rootly { config, storage, ini}
    }
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn init(data_path: Option<&str>, storage: Box<dyn Storage>) -> Rootly {
        Rootly::new(data_path, storage)
    }

    pub fn get_mutable_logs_file_name(&self) -> &String {
        self.ini.get_logs_file_name()
    }

}