use crate::dto::Card;
use std::fmt::Error;

pub trait Creator {
    fn create(&self, dto: crate::dto::Card) -> Result<crate::dto::Card, Error>;
}

pub(crate) struct Service {}

impl Service {
    pub(crate) fn new() -> Service {
        Service {}
    }
}

impl Creator for Service {
    fn create(&self, _: Card) -> Result<Card, Error> {
        unimplemented!()
    }
}
