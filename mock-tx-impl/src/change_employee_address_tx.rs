use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::ChangeEmployeeAddressTx;

pub struct ChangeEmployeeAddressTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub address: String,
}
impl HavePayrollDao<()> for ChangeEmployeeAddressTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeAddressTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeAddressTx::execute(self, self.emp_id, &self.address)
            .map(|_| ())
            .run(ctx)
    }
}
