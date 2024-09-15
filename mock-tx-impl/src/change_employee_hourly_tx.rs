use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::classification::ChangeEmployeeHourlyTx;

pub struct ChangeEmployeeHourlyTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub hourly_rate: f32,
}
impl HavePayrollDao<()> for ChangeEmployeeHourlyTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeHourlyTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeHourlyTx::execute(self, self.emp_id, self.hourly_rate)
            .map(|_| ())
            .run(ctx)
    }
}
