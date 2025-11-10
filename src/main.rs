mod generic_record_parser;
mod pc_financial_credit_record_parser;
mod tangerine_credit_record_parser;
mod tangerine_debit_record_parser;
mod transaction;

use crate::pc_financial_credit_record_parser::PcFinancialCreditRecordParser;
use crate::tangerine_credit_record_parser::TangerineCreditRecordParser;
use crate::tangerine_debit_record_parser::TangerineDebitRecordParser;
use crate::transaction::TransactionParser;
use csv::{Reader, StringRecord};
use std::fs;

fn main() {
    let folder_with_transactions = "/home/potato/Documents/Financial/Transactions/sample/";

    read_files_in_folder(folder_with_transactions);
}

fn read_files_in_folder(folder: &str) {
    let file_paths = fs::read_dir(folder);

    match file_paths {
        Ok(paths) => {
            for path in paths {
                match path {
                    Ok(path) => match Reader::from_path(path.path()) {
                        Ok(mut reader) => {
                            match reader.headers() {
                                Ok(headers) => match get_record_parser(headers) {
                                    Ok(record_parser) => {
                                        for record in reader.records() {
                                            match record {
                                                Ok(record) => {
                                                    let transaction =
                                                        record_parser.parse_transaction(&record);
                                                    match transaction {
                                                        Ok(mut transaction) => {
                                                            transaction.set_dataset_name(Some(
                                                                path.path().display().to_string(),
                                                            ));
                                                            println!("{:?}", transaction);
                                                        }
                                                        Err(error) => {
                                                            eprintln!("{:?}", error);
                                                        }
                                                    }
                                                }
                                                Err(error) => {
                                                    eprintln!("Error parsing record {}", error);
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        eprintln!("{}", error);
                                    }
                                },
                                Err(e) => eprintln!(
                                    "Error reading headers from [{}]. {}",
                                    path.path().display(),
                                    e
                                ),
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading file. {}", e);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading path. {}", e);
                    }
                }
            }
        }
        Err(e) => {
            panic!("Cannot read folder [{}]. {}", folder, e);
        }
    }
}

pub fn get_record_parser(csv_headers: &StringRecord) -> Result<Box<dyn TransactionParser>, String> {
    let pc_financial_credit_header: StringRecord =
        StringRecord::from(crate::pc_financial_credit_record_parser::PC_FINANCIAL_CREDIT_HEADERS);
    let tangerine_debit_header: StringRecord =
        StringRecord::from(crate::tangerine_debit_record_parser::TANGERINE_DEBIT_HEADERS);
    let tangerine_credit_header: StringRecord =
        StringRecord::from(crate::tangerine_credit_record_parser::TANGERINE_CREDIT_HEADERS);
    if csv_headers.eq(&pc_financial_credit_header) {
        Ok(Box::new(PcFinancialCreditRecordParser::new(csv_headers)))
    } else if csv_headers.eq(&tangerine_debit_header) {
        Ok(Box::new(TangerineDebitRecordParser::new(csv_headers)))
    } else if csv_headers.eq(&tangerine_credit_header) {
        Ok(Box::new(TangerineCreditRecordParser::new(csv_headers)))
    } else {
        Err(format!(
            "Record parser is not implemented for [{:?}]",
            csv_headers
        ))
    }
}
