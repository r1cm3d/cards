use crate::domain::card;
use crate::protocol;
use actix_web::{web, HttpResponse};

pub async fn create(
    service: web::Data<Box<dyn card::Creator>>,
    payload: web::Json<protocol::Card>,
) -> HttpResponse {
    let dto: protocol::Card = payload.into_inner();

    match service.create(dto) {
        Ok(card) => HttpResponse::Ok().json(card),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

pub static SCOPE: &str = "/cards";

#[cfg(test)]
mod tests {
    use crate::domain::card::Creator;
    use crate::protocol;
    use crate::protocol::{Card, ValidationError};
    use actix_web::web::{Data, Json};
    use mockall::mock;
    use mockall::predicate::eq;
    use std::str;

    mock! {
            Creator {}
            impl Creator for Creator {
               fn create(&self, card: crate::protocol::Card) -> Result<crate::protocol::Card, protocol::ValidationError>;
            }
    }

    #[actix_rt::test]
    async fn must_call_card_service_success() {
        let exp: Result<Card, protocol::ValidationError> = Ok(a_persisted_card());
        let act = call(&exp).await;
        let act = serde_json::from_str::<Card>(&act).expect("Failed to parse body into Card json");
        let exp = exp.unwrap();

        assert_eq!(exp, act)
    }

    #[actix_rt::test]
    async fn must_call_card_service_validation_error() {
        let exp: Result<Card, protocol::ValidationError> = Err(a_validation_error());
        let act = call(&exp).await;
        let act = serde_json::from_str::<ValidationError>(&act)
            .expect("Failed to parse body into ValidationError json");
        let exp = exp.unwrap_err();

        assert_eq!(exp, act)
    }

    async fn call(exp: &Result<Card, protocol::ValidationError>) -> String {
        let mut mock = MockCreator::new();
        mock.expect_create()
            .with(eq(a_input_card()))
            .return_const(exp.clone());
        let response = super::create(Data::new(Box::new(mock)), Json(a_input_card())).await;
        let act = match response.body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        let act = str::from_utf8(act).expect("Failed to parse Body::Bytes into str");

        return String::from(act);
    }

    fn a_input_card() -> Card {
        Card {
            id: "".to_string(),
            customer_id: String::from("29ce6541-302b-405e-9dfe-549934d4e4b2"),
            org_id: String::from("876ce143-6fcb-4c17-aaf1-f02c1d3654ce"),
            program_id: String::from("00c9e86a-8d55-4a95-884b-4a6faeb9289e"),
            account_id: String::from("a2d46c49-262e-431d-8f1a-ff5b18b44982"),
            printed_name: String::from("Baker Mayfield"),
            password: String::from("0781"),
            expiration_date: "".to_string(),
            issuing_date: "".to_string(),
            pan: "".to_string(),
            kind: "".to_string(),
            status: "".to_string(),
            cvv: "".to_string(),
        }
    }

    fn a_persisted_card() -> Card {
        Card {
            id: String::from("29ce6541-302b-405e-9dfe-549934d4e4b2"),
            customer_id: String::from("29ce6541-302b-405e-9dfe-549934d4e4b2"),
            org_id: String::from("876ce143-6fcb-4c17-aaf1-f02c1d3654ce"),
            program_id: String::from("00c9e86a-8d55-4a95-884b-4a6faeb9289e"),
            account_id: String::from("a2d46c49-262e-431d-8f1a-ff5b18b44982"),
            printed_name: String::from("Baker Mayfield"),
            password: String::from("0781"),
            expiration_date: String::from("0702"),
            issuing_date: String::from("1997-07-16T19:20+01:00"),
            pan: String::from("5214330278318136"),
            kind: String::from("PLASTIC"),
            status: String::from("ENABLED"),
            cvv: String::from("945"),
        }
    }

    fn a_validation_error() -> ValidationError {
        ValidationError::new(
            String::from("a_invalid_field"),
            String::from("a_invalid_value"),
        )
    }
}
