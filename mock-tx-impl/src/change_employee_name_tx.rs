use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::ChangeEmployeeNameTx;

pub struct ChangeEmployeeNameTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
}
impl HavePayrollDao<()> for ChangeEmployeeNameTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeNameTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeNameTx::execute(self, self.emp_id, &self.name)
            .map(|_| ())
            .run(ctx)
    }
}
