use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use abstract_tx::{ChangeAffiliationTx, UsecaseError};
use dao::{HavePayrollDao, PayrollDao};
use payroll_domain::{EmployeeId, MemberId};
use payroll_impl::AffiliationImpl;

pub trait ChangeUnionMemberTx<Ctx>: ChangeAffiliationTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        member_id: MemberId,
        dues: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeAffiliationTx::execute(
            self,
            emp_id,
            move |ctx, _| {
                self.dao()
                    .add_union_member(member_id, emp_id)
                    .run(ctx)
                    .map_err(UsecaseError::AddUnionMemberFailed)
            },
            Rc::new(RefCell::new(AffiliationImpl::Union {
                member_id,
                dues,
                service_charges: vec![],
            })),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeUnionMemberTx<Ctx> for T where T: HavePayrollDao<Ctx> {}
