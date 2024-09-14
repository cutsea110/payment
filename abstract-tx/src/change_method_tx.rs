use std::{cell::RefCell, rc::Rc};

use crate::change_employee_tx::ChangeEmployeeTx;
use crate::error::UsecaseError;
use payroll_domain::{EmployeeId, PaymentMethod};

pub trait ChangeEmployeePaymentMethodTx<Ctx>: ChangeEmployeeTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        method: Rc<RefCell<dyn PaymentMethod>>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTx::execute(self, emp_id, |_, emp| {
            emp.set_method(method);
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeePaymentMethodTx<Ctx> for T where T: ChangeEmployeeTx<Ctx> {}
