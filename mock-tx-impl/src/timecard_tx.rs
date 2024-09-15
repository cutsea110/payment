use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::TimeCardTx;

pub struct TimeCardTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub date: NaiveDate,
    pub hours: f32,
}
impl HavePayrollDao<()> for TimeCardTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for TimeCardTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        TimeCardTx::execute(self, self.emp_id, self.date, self.hours)
            .map(|_| ())
            .run(ctx)
    }
}
