use super::IpcReader;

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