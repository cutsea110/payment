use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::classification::ChangeEmployeeSalariedTx;

pub struct ChangeEmployeeSalariedTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub salary: f32,
}
impl HavePayrollDao<()> for ChangeEmployeeSalariedTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeSalariedTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeSalariedTx::execute(self, self.emp_id, self.salary)
            .map(|_| ())
            .run(ctx)
    }
}
