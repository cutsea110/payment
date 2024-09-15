use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::AddHourlyEmployeeTx;

pub struct AddHourlyEmployeeTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
    pub address: String,
    pub hourly_rate: f32,
}
impl HavePayrollDao<()> for AddHourlyEmployeeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for AddHourlyEmployeeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        AddHourlyEmployeeTx::execute(
            self,
            self.emp_id,
            &self.name,
            &self.address,
            self.hourly_rate.clone(),
        )
        .map(|_| ())
        .run(ctx)
    }
}
