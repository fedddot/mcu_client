pub trait IpcReader<Data, Error> {
    fn read_data(&mut self) -> Result<Data, Error>;
}

pub trait IpcWriter<Data, Error> {
    fn write_data(&mut self, data: &Data) -> Result<(), Error>;
}