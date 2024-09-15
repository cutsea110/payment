use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::method::ChangeEmployeeMailTx;

pub struct ChangeEmployeeMailTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub address: String,
}
impl HavePayrollDao<()> for ChangeEmployeeMailTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeMailTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeMailTx::execute(self, self.emp_id, &self.address)
            .map(|_| ())
            .run(ctx)
    }
}
