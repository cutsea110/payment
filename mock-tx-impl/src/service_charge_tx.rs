use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::MemberId;
use tx_app::Transaction;
use tx_impl::affiliation::ServiceChargeTx;

pub struct ServiceChargeTxImpl {
    pub db: MockDb,

    pub member_id: MemberId,
    pub date: NaiveDate,
    pub amount: f32,
}
impl HavePayrollDao<()> for ServiceChargeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ServiceChargeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ServiceChargeTx::execute(self, self.member_id, self.date, self.amount)
            .map(|_| ())
            .run(ctx)
    }
}
