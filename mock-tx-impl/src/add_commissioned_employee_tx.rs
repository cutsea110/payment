use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::AddCommissionedEmployeeTx;

pub struct AddCommissionedEmployeeTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
    pub address: String,
    pub salary: f32,
    pub commission_rate: f32,
}
impl HavePayrollDao<()> for AddCommissionedEmployeeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for AddCommissionedEmployeeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        AddCommissionedEmployeeTx::execute(
            self,
            self.emp_id,
            &self.name,
            &self.address,
            self.salary.clone(),
            self.commission_rate.clone(),
        )
        .map(|_| ())
        .run(ctx)
    }
}
