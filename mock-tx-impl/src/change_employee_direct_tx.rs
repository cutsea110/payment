use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::method::ChangeEmployeeDirectTx;

pub struct ChangeEmployeeDirectTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub bank: String,
    pub account: String,
}
impl HavePayrollDao<()> for ChangeEmployeeDirectTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeDirectTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeDirectTx::execute(self, self.emp_id, &self.bank, &self.account)
            .map(|_| ())
            .run(ctx)
    }
}
