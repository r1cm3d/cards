use crate::protocol;
use uuid::Uuid;
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use std::format;
use std::fmt::Error;

enum Kind {
    Plastic,
    Recurring,
    Temporary,
}

impl Kind {
    fn from(description: &str) -> Result<Kind, String>  {
        match description.to_uppercase().as_str() {
            "PLASTIC" => Ok(Kind::Plastic),
            "RECURRING" => Ok(Kind::Recurring),
            "TEMPORARY" => Ok(Kind::Temporary),
            _ => Err(format!("Unknown kind {}", description))
        }
    }
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
    expiration_date: String,
    issuing_date: chrono::NaiveDateTime,
    pan: String,
    kind: Kind,
    status: Status,
    cvv: String,
}



impl Entity {
    fn from(card: protocol::Card) -> Result<Entity, protocol::ValidationError> {
        macro_rules! validate_uuid_field {
        ($field:tt, $field_str:expr) => {
            let $field = match Uuid::parse_str(card.$field.as_str()) {
                Ok(ci) => ci, // TODO: check how to evaluate field_str
                Err(err) => return Err(protocol::ValidationError::new(String::from($field_str), card.$field.clone()))
            };
        }}

        macro_rules! validate_str_field {
        ($field:tt, $field_str:expr) => {
            let $field = match card.$field.is_empty() { // TODO: the same above
                true => return Err(protocol::ValidationError::new(String::from($field_str), card.$field.clone())),
                false => card.$field
            };
        }}

        validate_uuid_field!(customer_id, "customer_id");
        validate_uuid_field!(org_id, "org_id");
        validate_uuid_field!(program_id, "program_id");
        validate_uuid_field!(account_id, "account_id");
        validate_str_field!(printed_name, "printed_name");
        validate_str_field!(password, "password");
        validate_str_field!(expiration_date, "expiration_date");

        let kind = match Kind::from(card.kind.as_str()) {
            Ok(k) => k,
            Err(msg) => return Err(protocol::ValidationError::new(String::from("kind"), card.kind))
        };

        validate_str_field!(cvv, "cvv");

        Ok(Entity{
            id: Default::default(),
            customer_id,
            org_id,
            program_id,
            account_id,
            printed_name,
            password,
            expiration_date,
            issuing_date: NaiveDateTime::parse_from_str("2020-04-12", "%Y-%m-%d").unwrap(),
            pan: "".to_string(),
            kind,
            status: Status::Enabled,
            cvv
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

    macro_rules! test_invalid_field {
    ($name:ident, $input:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let svc = Service::new();

            let act = svc.create($input).unwrap_err();

            assert_eq!(act, $exp);
        }
    }}

    test_invalid_field!(test_invalid_customer_id, a_card_without_customer_id(), empty_error("customer_id"));
    test_invalid_field!(test_invalid_org_id, a_card_without_org_id(), empty_error("org_id"));
    test_invalid_field!(test_invalid_program_id, a_card_without_program_id(), empty_error("program_id"));
    test_invalid_field!(test_invalid_printed_name, a_card_without_printed_name(), empty_error("printed_name"));
    test_invalid_field!(test_invalid_password, a_card_without_password(), empty_error("password"));
    test_invalid_field!(test_invalid_expiration_date, a_card_without_expiration_date(), empty_error("expiration_date"));
    test_invalid_field!(test_invalid_kind, a_card_without_kind(), empty_error("kind"));
    test_invalid_field!(test_invalid_cvv, a_card_without_cvv(), empty_error("cvv"));

    // TODO: test pattern for printed name
    // TODO: test pattern for password
    // TODO: test pattern for expiration date
    // TODO: test pattern for cvv
    // TODO: test valid creation

    // TODO: extract those functions to macro
    fn a_card_without_customer_id() -> protocol::Card {
        protocol::Card{
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
        }
    }

    fn a_card_without_org_id() -> protocol::Card {
        protocol::Card{
            id: "".to_string(),
            customer_id: "a3643446-76fc-4516-8e43-bb6600ca118e".to_string(),
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
        }
    }

    fn a_card_without_program_id() -> protocol::Card {
        protocol::Card{
            id: "".to_string(),
            customer_id: "a3643446-76fc-4516-8e43-bb6600ca118e".to_string(),
            org_id: "3ee15c70-b7b4-4b87-ba43-38eba70f98c4".to_string(),
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
        }
    }

    fn a_card_without_account_id() -> protocol::Card {
        protocol::Card{
            id: "".to_string(),
            customer_id: "a3643446-76fc-4516-8e43-bb6600ca118e".to_string(),
            org_id: "3ee15c70-b7b4-4b87-ba43-38eba70f98c4".to_string(),
            program_id: "c0a4cc71-5c11-43cb-b74f-2b577012449f".to_string(),
            account_id: "".to_string(),
            printed_name: "".to_string(),
            password: "".to_string(),
            expiration_date: "".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "".to_string(),
            status: "".to_string(),
            cvv: "".to_string()
        }
    }

    fn a_card_without_printed_name() -> protocol::Card {
        protocol::Card{
            id: "".to_string(),
            customer_id: "a3643446-76fc-4516-8e43-bb6600ca118e".to_string(),
            org_id: "3ee15c70-b7b4-4b87-ba43-38eba70f98c4".to_string(),
            program_id: "c0a4cc71-5c11-43cb-b74f-2b577012449f".to_string(),
            account_id: "ba3df3ae-1da8-4b0a-be8c-e9f903d1f7de".to_string(),
            printed_name: "".to_string(),
            password: "".to_string(),
            expiration_date: "".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "".to_string(),
            status: "".to_string(),
            cvv: "".to_string()
        }
    }

    fn a_card_without_password() -> protocol::Card {
        protocol::Card{
            id: "".to_string(),
            customer_id: "a3643446-76fc-4516-8e43-bb6600ca118e".to_string(),
            org_id: "3ee15c70-b7b4-4b87-ba43-38eba70f98c4".to_string(),
            program_id: "c0a4cc71-5c11-43cb-b74f-2b577012449f".to_string(),
            account_id: "ba3df3ae-1da8-4b0a-be8c-e9f903d1f7de".to_string(),
            printed_name: "RICARDO MEDEIROS".to_string(),
            password: "".to_string(),
            expiration_date: "".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "".to_string(),
            status: "".to_string(),
            cvv: "".to_string()
        }
    }

    fn a_card_without_expiration_date() -> protocol::Card {
        protocol::Card{
            id: "".to_string(),
            customer_id: "a3643446-76fc-4516-8e43-bb6600ca118e".to_string(),
            org_id: "3ee15c70-b7b4-4b87-ba43-38eba70f98c4".to_string(),
            program_id: "c0a4cc71-5c11-43cb-b74f-2b577012449f".to_string(),
            account_id: "ba3df3ae-1da8-4b0a-be8c-e9f903d1f7de".to_string(),
            printed_name: "RICARDO MEDEIROS".to_string(),
            password: "321421".to_string(),
            expiration_date: "".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "".to_string(),
            status: "".to_string(),
            cvv: "".to_string()
        }
    }

    fn a_card_without_kind() -> protocol::Card {
        protocol::Card{
            id: "".to_string(),
            customer_id: "a3643446-76fc-4516-8e43-bb6600ca118e".to_string(),
            org_id: "3ee15c70-b7b4-4b87-ba43-38eba70f98c4".to_string(),
            program_id: "c0a4cc71-5c11-43cb-b74f-2b577012449f".to_string(),
            account_id: "ba3df3ae-1da8-4b0a-be8c-e9f903d1f7de".to_string(),
            printed_name: "RICARDO MEDEIROS".to_string(),
            password: "321421".to_string(),
            expiration_date: "0724".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "".to_string(),
            status: "".to_string(),
            cvv: "".to_string()
        }
    }

    fn a_card_without_cvv() -> protocol::Card {
        protocol::Card{
            id: "".to_string(),
            customer_id: "a3643446-76fc-4516-8e43-bb6600ca118e".to_string(),
            org_id: "3ee15c70-b7b4-4b87-ba43-38eba70f98c4".to_string(),
            program_id: "c0a4cc71-5c11-43cb-b74f-2b577012449f".to_string(),
            account_id: "ba3df3ae-1da8-4b0a-be8c-e9f903d1f7de".to_string(),
            printed_name: "RICARDO MEDEIROS".to_string(),
            password: "321421".to_string(),
            expiration_date: "2010-11-12T13:14:15Z".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "PLASTIC".to_string(),
            status: "".to_string(),
            cvv: "".to_string()
        }
    }

    fn empty_error(field: &str) -> protocol::ValidationError {
        protocol::ValidationError::new(String::from(field), String::from(""))
    }
}
