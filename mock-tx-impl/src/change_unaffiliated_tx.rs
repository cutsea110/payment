use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::affiliation::ChangeUnaffiliatedTx;

pub struct ChangeUnaffiliatedTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
}
impl HavePayrollDao<()> for ChangeUnaffiliatedTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeUnaffiliatedTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeUnaffiliatedTx::execute(self, self.emp_id)
            .map(|_| ())
            .run(ctx)
    }
}
