use crate::transaction::{Transaction, TransactionParser};
use chrono::{NaiveDate, NaiveDateTime};
use csv::StringRecord;
use rust_decimal::Decimal;
use std::str::FromStr;
pub struct GenericRecordParser {
    amount_field_index: u8,
    datetime_field_index: u8,
    description_positions: Vec<u8>,
    datetime_format: String,
}

impl GenericRecordParser {
    pub fn new(
        amount_field_index: u8,
        datetime_field_index: u8,
        description_positions: Vec<u8>,
        datetime_format: String,
    ) -> Self {
        GenericRecordParser {
            amount_field_index,
            datetime_field_index,
            description_positions,
            datetime_format,
        }
    }
}
impl TransactionParser for GenericRecordParser {
    fn parse_transaction(&self, csv_record: &StringRecord) -> Result<Transaction, Transaction> {
        let mut amount: Decimal = Decimal::default();
        let mut datetime: NaiveDateTime = NaiveDateTime::default();
        let mut description: String = String::new();
        let mut parsing_error_description: String = String::new();
        let mut parsing_error: bool = false;

        match csv_record.get(self.amount_field_index as usize) {
            Some(a) => match Decimal::from_str(a) {
                Ok(amount_parsed) => amount = amount_parsed,
                Err(e) => {
                    parsing_error = true;
                    parsing_error_description.push_str(e.to_string().as_str());
                }
            },
            None => {
                parsing_error = true;
                parsing_error_description.push_str("Missing value for the amount field");
            }
        }

        match csv_record.get(self.datetime_field_index as usize) {
            Some(a) => match NaiveDateTime::parse_from_str(a, self.datetime_format.as_str()) {
                Ok(date_parsed) => datetime = date_parsed,
                Err(e) => match NaiveDate::parse_from_str(a, self.datetime_format.as_str()) {
                    Ok(date_parsed) => datetime = NaiveDateTime::from(date_parsed),
                    Err(ee) => {
                        parsing_error = true;
                        parsing_error_description.push_str(
                            format!(
                                "Cannot parse [{}] from field at index [{}]. {}; {}",
                                self.datetime_format,
                                self.datetime_field_index,
                                e.to_string().as_str(),
                                ee.to_string().as_str()
                            )
                            .as_str(),
                        );
                    }
                },
            },
            None => {
                parsing_error = true;
                parsing_error_description.push_str(
                    format!(
                        "Missing value for datetime field at index [{}]",
                        self.datetime_field_index
                    )
                    .as_str(),
                );
            }
        }

        for description_filed_index in self.description_positions.iter() {
            match csv_record.get(*description_filed_index as usize) {
                Some(a) => {
                    description.push_str(a.to_string().as_str());
                    description.push(';')
                }
                None => {
                    parsing_error = true;
                    parsing_error_description.push_str(
                        format!(
                            "Failed to parse description field at position {}",
                            description_filed_index
                        )
                        .as_str(),
                    );
                }
            }
        }

        if parsing_error {
            Err(Transaction::new(
                amount,
                datetime,
                description,
                Some(parsing_error_description.to_string()),
            ))
        } else {
            Ok(Transaction::new(amount, datetime, description, None))
        }
    }
}
