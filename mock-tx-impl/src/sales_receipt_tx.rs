use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::SalesReceiptTx;

pub struct SalesReceiptTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub date: NaiveDate,
    pub amount: f32,
}
impl HavePayrollDao<()> for SalesReceiptTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for SalesReceiptTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        SalesReceiptTx::execute(self, self.emp_id, self.date, self.amount)
            .map(|_| ())
            .run(ctx)
    }
}
