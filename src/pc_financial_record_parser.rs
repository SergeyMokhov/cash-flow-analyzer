use crate::transaction::{Transaction, TransactionParser};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use csv::StringRecord;
use rust_decimal::Decimal;
use std::str::FromStr;

const AMOUNT: &str = "Amount";
const DESCRIPTION: &str = "Description";
const TYPE: &str = "Type";
const CARD_HOLDER_NAME: &str = "Card Holder Name";
const DATE: &str = "Date";
const TIME: &str = "Time";
pub static PC_FINANCIAL_HEADERS: &[&str] =
    &[DESCRIPTION, TYPE, CARD_HOLDER_NAME, DATE, TIME, AMOUNT];

pub struct PcFinancialRecordParser {
    amount_position: u8,
    date_position: u8,
    time_position: u8,
    description_positions: Vec<u8>,
}

impl Default for PcFinancialRecordParser {
    fn default() -> PcFinancialRecordParser {
        PcFinancialRecordParser {
            amount_position: 5,
            date_position: 3,
            time_position: 4,
            description_positions: vec![0, 1, 2],
        }
    }
}

impl PcFinancialRecordParser {
    pub fn new(csv_header: &StringRecord) -> Self {
        let mut amount_position: u8 = 0;
        let mut date_position: u8 = 0;
        let mut time_position: u8 = 0;
        let mut description_positions: Vec<u8> = Vec::new();

        for i in 0..csv_header.len() {
            match csv_header.get(i) {
                Some(AMOUNT) => amount_position = i as u8,
                Some(DATE) => date_position = i as u8,
                Some(TIME) => time_position = i as u8,
                _ => description_positions.push(i as u8),
            }
        }

        PcFinancialRecordParser {
            amount_position,
            date_position,
            time_position,
            description_positions,
        }
    }
}
impl TransactionParser for PcFinancialRecordParser {
    fn parse_transaction(&self, csv_record: &StringRecord) -> Result<Transaction, Transaction> {
        let mut amount: Decimal = Decimal::default();
        let mut date: NaiveDate = NaiveDate::default();
        let mut time: NaiveTime = NaiveTime::default();
        let mut description: String = String::new();
        let mut parsing_error_description: String = String::new();
        let mut parsing_error: bool = false;

        match csv_record.get(self.amount_position as usize) {
            Some(a) => match Decimal::from_str(a) {
                Ok(amount_parsed) => amount = amount_parsed,
                Err(e) => {
                    parsing_error = true;
                    parsing_error_description.push_str(e.to_string().as_str());
                }
            },
            None => {
                parsing_error = true;
                parsing_error_description
                    .push_str(format!("Missing value for {} field", AMOUNT).as_str());
            }
        }

        match csv_record.get(self.date_position as usize) {
            Some(a) => match NaiveDate::parse_from_str(a, "%m/%d/%Y") {
                Ok(date_parsed) => date = date_parsed,
                Err(e) => {
                    parsing_error = true;
                    parsing_error_description.push_str(e.to_string().as_str());
                }
            },
            None => {
                parsing_error = true;
                parsing_error_description
                    .push_str(format!("Missing value for {} field", DATE).as_str());
            }
        }

        match csv_record.get(self.time_position as usize) {
            Some(a) => match NaiveTime::parse_from_str(a, "%I:%M %p") {
                Ok(time_parsed) => time = time_parsed,
                Err(e) => {
                    parsing_error = true;
                    parsing_error_description.push_str(e.to_string().as_str());
                }
            },
            None => {
                parsing_error = true;
                parsing_error_description
                    .push_str(format!("Missing value for {} field", TIME).as_str());
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
                NaiveDateTime::new(date, time),
                description,
                Some(parsing_error_description.to_string()),
            ))
        } else {
            Ok(Transaction::new(
                amount,
                NaiveDateTime::new(date, time),
                description,
                None,
            ))
        }
    }
}
