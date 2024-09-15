use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::{EmployeeId, MemberId};
use tx_app::Transaction;
use tx_impl::affiliation::ChangeUnionMemberTx;

pub struct ChangeUnionMemberTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub member_id: MemberId,
    pub dues: f32,
}
impl HavePayrollDao<()> for ChangeUnionMemberTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeUnionMemberTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeUnionMemberTx::execute(self, self.emp_id, self.member_id, self.dues)
            .map(|_| ())
            .run(ctx)
    }
}
