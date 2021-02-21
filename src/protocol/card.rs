use indoc::indoc;
use serde::{Serialize,Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) customer_id: String,
    #[serde(default)]
    pub(crate) org_id: String,
    #[serde(default)]
    pub(crate) program_id: String,
    #[serde(default)]
    pub(crate) account_id: String,
    #[serde(default)]
    pub(crate) printed_name: String,
    #[serde(default)]
    pub(crate) password: String,
    #[serde(default)]
    pub(crate) expiration_date: String,
    #[serde(default)]
    pub(crate) issuing_date: String,
    #[serde(default)]
    pub(crate) pan: String,
    #[serde(default)]
    pub(crate) kind: String,
    #[serde(default)]
    pub(crate) status: String,
    #[serde(default)]
    pub(crate) cvv: String,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            indoc! {"
                    {{
                        id: {},
                        customer_id: {},
                        org_id: {},
                        program_id: {},
                        account_id: {},
                        printed_name: {},
                        password: {},
                        expiration_date: {},
                        issuing_date: {},
                        pan: {},
                        kind: {},
                        status: {},
                        cvv: {},
                    }}
                  "},
            self.id,
            self.customer_id,
            self.org_id,
            self.program_id,
            self.account_id,
            self.printed_name,
            self.password,
            self.expiration_date,
            self.issuing_date,
            self.pan,
            self.kind,
            self.status,
            self.cvv
        )
    }
}
