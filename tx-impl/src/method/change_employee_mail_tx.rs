use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeEmployeePaymentMethodTx, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::PaymentMethodImpl;

pub trait ChangeEmployeeMailTx<Ctx>: ChangeEmployeePaymentMethodTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        address: &str,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeePaymentMethodTx::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(PaymentMethodImpl::Mail {
                address: address.to_string(),
            })),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeeMailTx<Ctx> for T where T: ChangeEmployeePaymentMethodTx<Ctx> {}
