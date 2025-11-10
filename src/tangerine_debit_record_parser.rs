use crate::generic_record_parser::GenericRecordParser;
use crate::transaction::{Transaction, TransactionParser};
use csv::StringRecord;

const DATE: &str = "Date";
const TRANSACTION: &str = "Transaction";
const NAME: &str = "Name";
const MEMO: &str = "Memo";
const AMOUNT: &str = "Amount";
const DATETIME_FORMAT: &str = "%m/%d/%Y";
pub static TANGERINE_DEBIT_HEADERS: &[&str] = &[DATE, TRANSACTION, NAME, MEMO, AMOUNT];

pub struct TangerineDebitRecordParser {
    generic_record_parser: GenericRecordParser,
}

impl Default for TangerineDebitRecordParser {
    fn default() -> TangerineDebitRecordParser {
        let amount_position: u8 = 4;
        let date_position: u8 = 0;
        let description_positions: Vec<u8> = vec![1, 2, 3];
        let parser = GenericRecordParser::new(
            amount_position,
            date_position,
            description_positions,
            DATETIME_FORMAT.to_string(),
        );

        TangerineDebitRecordParser {
            generic_record_parser: parser,
        }
    }
}

impl TangerineDebitRecordParser {
    pub fn new(csv_header: &StringRecord) -> Self {
        let mut amount_position: u8 = 0;
        let mut date_position: u8 = 0;
        let mut description_positions: Vec<u8> = Vec::new();

        for i in 0..csv_header.len() {
            match csv_header.get(i) {
                Some(AMOUNT) => amount_position = i as u8,
                Some(DATE) => date_position = i as u8,
                _ => description_positions.push(i as u8),
            }
        }

        let parser = GenericRecordParser::new(
            amount_position,
            date_position,
            description_positions,
            DATETIME_FORMAT.to_string(),
        );

        TangerineDebitRecordParser {
            generic_record_parser: parser,
        }
    }
}
impl TransactionParser for TangerineDebitRecordParser {
    fn parse_transaction(&self, csv_record: &StringRecord) -> Result<Transaction, Transaction> {
        self.generic_record_parser.parse_transaction(csv_record)
    }
}
