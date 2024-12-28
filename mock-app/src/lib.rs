use std::path::PathBuf;

use mock_db::MockDb;
use mock_tx_impl::TransactionFactoryImpl;
use text_parser_tx_source::TextParserTransactionSource;
use tx_app::{TransactionApplication, TransactionSource};

#[derive(Debug, Clone)]
pub struct TestPayrollApp {
    db: MockDb,
    file_path: PathBuf,
}
impl TestPayrollApp {
    pub fn new(file_name: &str) -> Self {
        Self {
            db: MockDb::new(),
            file_path: file_name.into(),
        }
    }
}
impl TransactionApplication<()> for TestPayrollApp {
    fn tx_source(&self) -> impl TransactionSource<()> {
        let input = std::fs::read_to_string(&self.file_path).expect("read script file");
        let tx_factory = TransactionFactoryImpl::new(self.db.clone());

        TextParserTransactionSource::new(tx_factory, input)
    }
}
