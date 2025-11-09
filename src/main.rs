mod pc_financial_record_parser;
mod transaction;

use crate::pc_financial_record_parser::PcFinancialRecordParser;
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
                    Ok(path) => {
                        // println!("{}", path.path().display());
                        match Reader::from_path(path.path()) {
                            Ok(mut reader) => {
                                match reader.headers() {
                                    Ok(headers) => match get_record_parser(headers) {
                                        Ok(record_parser) => {
                                            for record in reader.records() {
                                                match record {
                                                    Ok(record) => {
                                                        let transaction = record_parser
                                                            .parse_transaction(&record);
                                                        println!("{:?}", transaction);
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
                                println!("{:?}", reader.headers().unwrap());
                            }
                            Err(e) => {
                                eprintln!("Error reading file. {}", e);
                            }
                        }
                    }
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

pub fn get_record_parser(
    csv_headers: &StringRecord,
) -> Result<Box<dyn TransactionParser>, String> {
    let pc_financial_header: StringRecord =
        StringRecord::from(crate::pc_financial_record_parser::PC_FINANCIAL_HEADERS);
    if csv_headers.eq(&pc_financial_header) {
        Ok(Box::new(PcFinancialRecordParser::new(csv_headers)))
    } else {
        Err(format!(
            "Record parser is not implemented for [{:?}]",
            csv_headers
        ))
    }
}
