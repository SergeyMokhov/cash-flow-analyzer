use chrono::NaiveDateTime;
use csv::StringRecord;
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Transaction {
    amount: Decimal,
    datetime: NaiveDateTime,
    description: String,
    parsing_error: Option<String>,
}

impl Transaction {
    pub fn new(
        amount: Decimal,
        datetime: NaiveDateTime,
        description: String,
        parsing_error: Option<String>,
    ) -> Transaction {
        Transaction {
            amount,
            datetime,
            description,
            parsing_error,
        }
    }
    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn datetime(&self) -> NaiveDateTime {
        self.datetime
    }

    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn parsing_error(&self) -> Option<&str> {
        self.parsing_error.as_deref()
    }

    pub fn set_amount(&mut self, amount: Decimal) {
        self.amount = amount;
    }

    pub fn set_datetime(&mut self, datetime: NaiveDateTime) {
        self.datetime = datetime;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_parsing_error(&mut self, parsing_error: String) {
        self.parsing_error = Some(parsing_error);
    }
}

pub trait TransactionParser {
    fn parse_transaction(&self, csv_record: &StringRecord) -> Result<Transaction, Transaction>;
}
