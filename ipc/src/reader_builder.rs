pub use super::IpcReader;

#[derive(Default)]
pub struct IpcReaderBuilder<RawData, Data, Error> {
    raw_data_reader: Option<Box<dyn IpcReader<RawData, Error>>>,
    raw_data_parser: Option<Fn(&RawData) -> Data>,
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

pub type RawDataParser<RawData, Data, Error> = dyn Fn(&RawData) -> Result<Data, Error>;
pub type RawDataReader<RawData, Error> = dyn IpcReader<RawData, Error>;

struct ComposedIpcReader<RawData, Data, Error> {
    raw_data_reader: Box<RawDataReader<RawData, Error>>,
    raw_data_parser: Box<RawDataParser<RawData, Data, Error>>,
}

impl<RawData, Data, Error> ComposedIpcReader<RawData, Data, Error> {
    pub fn new(
        raw_data_reader: Box<RawDataReader<RawData, Error>>,
        raw_data_parser: Box<RawDataParser<RawData, Data, Error>>,
    ) -> Self {
        Self {
            raw_data_reader,
            raw_data_parser,
        }
    }
}

impl<RawData, Data, Error> IpcReader<Data, Error> for ComposedIpcReader<RawData, Data, Error> {
    fn read_data(&mut self) -> Result<Data, Error> {
        let raw_data = self.raw_data_reader.read_data()?;
        let parsed_data = self.raw_data_parser(&raw_data)?;
        todo!("NOT IMPLEMENTED")
    }
}

#[cfg(test)]
mod reader_builder_tests;