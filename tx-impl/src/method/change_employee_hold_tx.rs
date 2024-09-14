use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeEmployeePaymentMethodTx, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::PaymentMethodImpl;

pub trait ChangeEmployeeHoldTx<Ctx>: ChangeEmployeePaymentMethodTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeePaymentMethodTx::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(PaymentMethodImpl::Hold)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeeHoldTx<Ctx> for T where T: ChangeEmployeePaymentMethodTx<Ctx> {}
