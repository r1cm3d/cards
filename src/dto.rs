use indoc::indoc;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct Card {
    customer_id: String,
    org_id: String,
    program_id: String,
    account_id: String,
    printed_name: String,
    password: String,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            indoc! {"
                    {{
                        customer_id: {},
                        org_id: {},
                        program_id: {},
                        account_id: {},
                        printed_name: {},
                        password: {}
                    }}
                  "},
            self.customer_id,
            self.org_id,
            self.program_id,
            self.account_id,
            self.printed_name,
            self.password
        )
    }
}
