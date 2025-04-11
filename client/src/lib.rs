pub trait ServiceClient<Request, Response, Error> {
    fn run_request(&mut self, request: &Request) -> Result<Response, Error>;
}