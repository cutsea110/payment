use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use abstract_tx::{ChangeAffiliationTx, UsecaseError};
use dao::{HavePayrollDao, PayrollDao};
use payroll_domain::EmployeeId;
use payroll_impl::AffiliationImpl;

pub trait ChangeUnaffiliatedTx<Ctx>: ChangeAffiliationTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeAffiliationTx::execute(
            self,
            emp_id,
            move |ctx, emp| {
                let member_id = emp
                    .get_affiliation()
                    .borrow()
                    .as_any()
                    .downcast_ref::<AffiliationImpl>()
                    .map_or(
                        Err(UsecaseError::UnexpectedAffiliation(format!(
                            "expected unaffiliated emp_id: {}",
                            emp_id
                        ))),
                        |a| Ok(a.get_member_id()),
                    )?;
                self.dao()
                    .remove_union_member(member_id)
                    .run(ctx)
                    .map_err(UsecaseError::RemoveUnionMemberFailed)
            },
            Rc::new(RefCell::new(AffiliationImpl::Unaffiliated)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeUnaffiliatedTx<Ctx> for T where T: HavePayrollDao<Ctx> {}
