mod data_provider;
pub mod storage_api;
mod model;
mod parsers;

use crate::model::header::Header;
use crate::model::ini::IniFile;
use crate::model::log::LogFile;
use crate::model::ReadableModel;
use config::{Config, Environment, File};
use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use std::path::Path;
use storage_api::Storage;

pub struct Rootly {
    config: Config,
    storage: Box<dyn Storage>,
    ini: IniFile,
}
impl Rootly {
    fn new(data_path: &String, storage: Box<dyn Storage>) -> Rootly {
        let mut config_builder = Config::builder().
            add_source(Environment::with_prefix("ROOTLY"));
        let data_path_obj:&Path;
        if !data_path.is_empty(){
            data_path_obj = Path::new(&data_path);
            let config_path = data_path_obj.join("rootly.toml");
            config_builder = config_builder.add_source(File::with_name(config_path.to_str().unwrap()));
            config_builder = config_builder.set_default("data_path", data_path.clone()).unwrap();
        }
        let config = config_builder.build().unwrap();
        let rootly_ini = storage.read_config();
        let ini:IniFile;
        if rootly_ini.is_none(){
            ini = IniFile::build_default();
            storage.write_config(ini.to_bytes());
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

    pub fn init(data_path: String, storage: Box<dyn Storage>) -> Rootly {

        Rootly::new(&data_path, storage)
    }

    pub fn write_trace_log(&self, trace_service_request: ExportTraceServiceRequest) {
        let traces = parsers::trace_parser(trace_service_request);
        println!("{:?}", traces);
    }

    pub fn write_log(&self, export_logs_service_request: ExportLogsServiceRequest, ) {
        let logs = parsers::logs_parser(export_logs_service_request);
        let mut log_file:LogFile = LogFile::new();
        for log in logs {
            log_file.add_log(log);
        }
        self.storage.write_mutable_logs(log_file.to_bytes());
    }

}




