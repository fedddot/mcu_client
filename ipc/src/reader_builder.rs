pub use super::IpcReader;

#[derive(Default)]
pub struct IpcReaderBuilder<RawData, Data, Error> {
    raw_data_reader: Option<Box<dyn IpcReader<Data, Error>>>,
    raw_data_parser: Option<Box<dyn Fn(&RawData) -> Data>>,
}

impl<RawData, Data, Error> IpcReaderBuilder<RawData, Data, Error> {
    pub fn set_raw_data_reader(&mut self, reader: Box<dyn IpcReader<Data, Error>>) {
        self.raw_data_reader = Some(reader);
    }

    pub fn set_raw_data_parser(&mut self, parser: Box<dyn Fn(&RawData) -> Data>) {
        self.raw_data_parser = Some(parser);
    }
    
    pub fn build(self) -> Result<Box<dyn IpcReader<Data, Error>>, String> {
        if self.raw_data_reader.is_none() || self.raw_data_parser.is_none() {
            return Err("ipc reader builder is not set up".to_string());
        }
        todo!("NOT IMPLEMENTED")
    }
}

mod composed_ipc_reader;