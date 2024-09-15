use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::AddSalaryEmployeeTx;

pub struct AddSalaryEmployeeTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
    pub address: String,
    pub salary: f32,
}
impl HavePayrollDao<()> for AddSalaryEmployeeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for AddSalaryEmployeeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        AddSalaryEmployeeTx::execute(
            self,
            self.emp_id,
            &self.name,
            &self.address,
            self.salary.clone(),
        )
        .map(|_| ())
        .run(ctx)
    }
}
