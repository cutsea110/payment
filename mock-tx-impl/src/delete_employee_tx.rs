use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::DeleteEmployeeTx;

pub struct DeleteEmployeeTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
}
impl HavePayrollDao<()> for DeleteEmployeeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for DeleteEmployeeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        DeleteEmployeeTx::execute(self, self.emp_id)
            .map(|_| ())
            .run(ctx)
    }
}
