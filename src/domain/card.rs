use crate::protocol;
use uuid::Uuid;
use chrono::{DateTime, NaiveDate, NaiveDateTime};

enum Kind {
    Plastic,
    Recurring,
    Temporary
}

enum Status {
    Enabled,
    Cancelled,
    Blocked
}


struct Entity {
    id: uuid::Uuid,
    customer_id: uuid::Uuid,
    org_id: uuid::Uuid,
    program_id: uuid::Uuid,
    account_id: uuid::Uuid,
    printed_name: String,
    password: String,
    expiration_date: chrono::NaiveDateTime,
    issuing_date: chrono::NaiveDateTime,
    pan: String,
    kind: Kind,
    status: Status,
    cvv: String,
}

impl Entity {
    fn from(card: protocol::Card) -> Result<Entity, protocol::ValidationError> {
        let customer_id = Uuid::parse_str(card.customer_id.as_str());

        let customer_id = match customer_id {
            Ok(ci) => ci,
            Err(err) => return Err(protocol::ValidationError::new(String::from("customer_id"), card.customer_id.clone()))
        };

        Ok(Entity{
            id: Default::default(),
            customer_id,
            org_id: Default::default(),
            program_id: Default::default(),
            account_id: Default::default(),
            printed_name: "".to_string(),
            password: "".to_string(),
            expiration_date: NaiveDateTime::parse_from_str("2020-04-12", "%Y-%m-%d").unwrap(),
            issuing_date: NaiveDateTime::parse_from_str("2020-04-12", "%Y-%m-%d").unwrap(),
            pan: "".to_string(),
            kind: Kind::Plastic,
            status: Status::Enabled,
            cvv: "".to_string()
        })
    }
}

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
    fn create(&self, card: protocol::Card) -> Result<protocol::Card, protocol::ValidationError> {
        let entity = Entity::from(card)?;

        Ok(protocol::Card{
            id: "".to_string(),
            customer_id: entity.customer_id.to_string(),
            org_id: "".to_string(),
            program_id: "".to_string(),
            account_id: "".to_string(),
            printed_name: "".to_string(),
            password: "".to_string(),
            expiration_date: "".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "".to_string(),
            status: "".to_string(),
            cvv: "".to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol;
    use super::*;

    //TODO: create macro to test all invalid fields

    #[test]
    fn test_invalid_customer_id() {
        let card = protocol::Card{
            id: "".to_string(),
            customer_id: "".to_string(),
            org_id: "".to_string(),
            program_id: "".to_string(),
            account_id: "".to_string(),
            printed_name: "".to_string(),
            password: "".to_string(),
            expiration_date: "".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "".to_string(),
            status: "".to_string(),
            cvv: "".to_string()
        };
        let svc = Service::new();

        let err = svc.create(card).unwrap_err();

        assert_eq!(err.field_name(), String::from("customer_id"));
        assert_eq!(err.inputted_value(), String::from(""));
    }

}
