use crate::protocol;
use uuid::Uuid;
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use std::format;
use std::fmt::Error;
use regex::Regex;

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

        macro_rules! validate_str_field_with_regex {
        ($field:tt, $regex:expr, $field_str:expr) => {
            let re = Regex::new($regex).unwrap();
            let $field = match re.is_match(&card.$field) {
                true => card.$field,
                false => return Err(protocol::ValidationError::new(String::from($field_str), card.$field.clone()))
            };
        }}

        validate_uuid_field!(customer_id, "customer_id");
        validate_uuid_field!(org_id, "org_id");
        validate_uuid_field!(program_id, "program_id");
        validate_uuid_field!(account_id, "account_id");
        validate_str_field_with_regex!(printed_name, r"^[A-Z\s]+$", "printed_name");
        validate_str_field_with_regex!(password, r"^\d{6}$", "password");
        validate_str_field_with_regex!(cvv, r"^\d{3}\d?$", "cvv");
        validate_str_field_with_regex!(expiration_date, r"^(0\d|1[0-2])\d{2}$", "expiration_date");

        let kind = match Kind::from(card.kind.as_str()) {
            Ok(k) => k,
            Err(msg) => return Err(protocol::ValidationError::new(String::from("kind"), card.kind))
        };


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
    test_invalid_field!(test_invalid_password, a_card_without_password(), invalid_error("password", ""));
    test_invalid_field!(test_invalid_kind, a_card_without_kind(), empty_error("kind"));
    test_invalid_field!(test_invalid_cvv, a_card_without_cvv(), invalid_error("cvv", ""));
    test_invalid_field!(test_invalid_printed_name_invalid_characters_number, a_card_with_invalid_printed_name("R1CARDO"), invalid_error("printed_name", "R1CARDO"));
    test_invalid_field!(test_invalid_printed_name_invalid_characters_cedilha, a_card_with_invalid_printed_name("RIÇARDO"), invalid_error("printed_name", "RIÇARDO"));
    test_invalid_field!(test_invalid_printed_name_invalid_characters_special_characters, a_card_with_invalid_printed_name("#RIC*RDO"), invalid_error("printed_name", "#RIC*RDO"));
    test_invalid_field!(test_invalid_password_with_letters, a_card_with_invalid_password("0912C8"), invalid_error("password", "0912C8"));
    test_invalid_field!(test_invalid_password_with_more_than_six_characters, a_card_with_invalid_password("091261128"), invalid_error("password", "091261128"));
    test_invalid_field!(test_invalid_cvv_with_letters, a_card_with_invalid_cvv("0B12"), invalid_error("cvv", "0B12"));
    test_invalid_field!(test_invalid_cvv_with_more_than_four_characters, a_card_with_invalid_cvv("61112"), invalid_error("cvv", "61112"));
    test_invalid_field!(test_invalid_empty_expiration_date, a_card_without_expiration_date(), invalid_error("expiration_date", ""));
    test_invalid_field!(test_invalid_expiration_date_with_letters, a_card_with_invalid_expiration_date("ABCEFG"), invalid_error("expiration_date", "ABCEFG"));
    test_invalid_field!(test_invalid_expiration_date_with_invalid_month, a_card_with_invalid_expiration_date("1300"), invalid_error("expiration_date", "1300"));

    // TODO: test valid creation

    // TODO: check if it is possible to extract these functions to a macro
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
            kind: "PLASTIC".to_string(),
            status: "".to_string(),
            cvv: "745".to_string()
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
            cvv: "512".to_string()
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
            expiration_date: "0724".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "PLASTIC".to_string(),
            status: "".to_string(),
            cvv: "".to_string()
        }
    }

    fn a_card_with_invalid_printed_name(invalid_printed_name: &str) -> protocol::Card {
        protocol::Card{
            id: "".to_string(),
            customer_id: "a3643446-76fc-4516-8e43-bb6600ca118e".to_string(),
            org_id: "3ee15c70-b7b4-4b87-ba43-38eba70f98c4".to_string(),
            program_id: "c0a4cc71-5c11-43cb-b74f-2b577012449f".to_string(),
            account_id: "ba3df3ae-1da8-4b0a-be8c-e9f903d1f7de".to_string(),
            printed_name: invalid_printed_name.to_string(),
            password: "321421".to_string(),
            expiration_date: "0724".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "PLASTIC".to_string(),
            status: "".to_string(),
            cvv: "123".to_string()
        }
    }

    fn a_card_with_invalid_password(invalid_password: &str) -> protocol::Card {
        protocol::Card{
            id: "".to_string(),
            customer_id: "a3643446-76fc-4516-8e43-bb6600ca118e".to_string(),
            org_id: "3ee15c70-b7b4-4b87-ba43-38eba70f98c4".to_string(),
            program_id: "c0a4cc71-5c11-43cb-b74f-2b577012449f".to_string(),
            account_id: "ba3df3ae-1da8-4b0a-be8c-e9f903d1f7de".to_string(),
            printed_name: "RICARDO".to_string(),
            password: invalid_password.to_string(),
            expiration_date: "0724".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "PLASTIC".to_string(),
            status: "".to_string(),
            cvv: "123".to_string()
        }
    }

    fn a_card_with_invalid_cvv(invalid_cvv: &str) -> protocol::Card {
        protocol::Card{
            id: "".to_string(),
            customer_id: "a3643446-76fc-4516-8e43-bb6600ca118e".to_string(),
            org_id: "3ee15c70-b7b4-4b87-ba43-38eba70f98c4".to_string(),
            program_id: "c0a4cc71-5c11-43cb-b74f-2b577012449f".to_string(),
            account_id: "ba3df3ae-1da8-4b0a-be8c-e9f903d1f7de".to_string(),
            printed_name: "RICARDO".to_string(),
            password: "072465".to_string(),
            expiration_date: "0724".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "PLASTIC".to_string(),
            status: "".to_string(),
            cvv: invalid_cvv.to_string()
        }
    }

    fn a_card_with_invalid_expiration_date(invalid_expiration_date: &str) -> protocol::Card {
        protocol::Card {
            id: "".to_string(),
            customer_id: "a3643446-76fc-4516-8e43-bb6600ca118e".to_string(),
            org_id: "3ee15c70-b7b4-4b87-ba43-38eba70f98c4".to_string(),
            program_id: "c0a4cc71-5c11-43cb-b74f-2b577012449f".to_string(),
            account_id: "ba3df3ae-1da8-4b0a-be8c-e9f903d1f7de".to_string(),
            printed_name: "RICARDO".to_string(),
            password: "517412".to_string(),
            expiration_date: invalid_expiration_date.to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "PLASTIC".to_string(),
            status: "".to_string(),
            cvv: "451".to_string()
        }
    }

    fn empty_error(field: &str) -> protocol::ValidationError {
        protocol::ValidationError::new(String::from(field), String::from(""))
    }

    fn invalid_error(field: &str, inputted_value: &str) -> protocol::ValidationError {
        protocol::ValidationError::new(String::from(field), String::from(inputted_value))
    }
}
