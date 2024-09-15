use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::classification::ChangeEmployeeCommissionedTx;

pub struct ChangeEmployeeCommissionedTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub salary: f32,
    pub commission_rate: f32,
}
impl HavePayrollDao<()> for ChangeEmployeeCommissionedTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeCommissionedTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeCommissionedTx::execute(self, self.emp_id, self.salary, self.commission_rate)
            .map(|_| ())
            .run(ctx)
    }
}
