use crate::protocol;

pub trait Creator {
    fn create(&self, dto: protocol::Card) -> Result<protocol::Card, protocol::ValidationError>;
}

pub(crate) struct Service {}

impl Service {
    pub(crate) fn new() -> Service {
        Service {}
    }
}

impl Creator for Service {
    fn create(&self, _: protocol::Card) -> Result<protocol::Card, protocol::ValidationError> {
        unimplemented!()
    }
}
