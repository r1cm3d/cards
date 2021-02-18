use crate::domain::card;
use crate::dto;
use actix_web::{web, HttpResponse, Responder};

pub async fn create(
    service: web::Data<Box<dyn card::Creator>>,
    payload: web::Json<dto::Card>,
) -> impl Responder {
    let dto: dto::Card = payload.into_inner();
    println!("DTO: {}", dto);

    //TODO: Fix it adjust the return
    service.create(dto);
    HttpResponse::Ok().body("OK")
}

pub static SCOPE: &str = "/cards";

#[cfg(test)]
mod tests {
    use std::fmt::Error;
    use actix_web::web;
    use actix_web::web::Data;
    use mockall::mock;
    use mockall::predicate::eq;
    use crate::domain::card::Creator;
    use crate::dto;

    #[actix_rt::test]
    async fn must_call_card_service_succes() {
        mock! {
            Creator {}
            impl Creator for Creator {
               fn create(&self, dto: crate::dto::Card) -> Result<crate::dto::Card, Error>;
            }
        }
        let mut mock = MockCreator::new();
        let exp: Result<dto::Card, Error> = Ok(dto::Card {
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
        });

         mock.expect_create()
             .with(eq(a_card()))
             .return_const(exp);

        //TODO: validate the return
        super::create(Data::new(Box::new(mock)),web::Json(a_card())).await;
    }

    fn a_card() -> dto::Card {
        dto::Card {
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
}
