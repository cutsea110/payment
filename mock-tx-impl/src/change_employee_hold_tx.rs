use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::method::ChangeEmployeeHoldTx;

pub struct ChangeEmployeeHoldTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
}
impl HavePayrollDao<()> for ChangeEmployeeHoldTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeHoldTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeHoldTx::execute(self, self.emp_id)
            .map(|_| ())
            .run(ctx)
    }
}
