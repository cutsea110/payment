use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use tx_app::Transaction;
use tx_impl::general::PaydayTx;

pub struct PaydayTxImpl {
    pub db: MockDb,

    pub pay_date: NaiveDate,
}
impl HavePayrollDao<()> for PaydayTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for PaydayTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        PaydayTx::execute(self, self.pay_date).map(|_| ()).run(ctx)
    }
}
