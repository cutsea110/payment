use std::{cell::RefCell, rc::Rc};

use crate::change_employee_tx::ChangeEmployeeTx;
use crate::error::UsecaseError;
use payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule};

pub trait ChangeEmployeePaymentClassificationTx<Ctx>: ChangeEmployeeTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        classification: Rc<RefCell<dyn PaymentClassification>>,
        schedule: Rc<RefCell<dyn PaymentSchedule>>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTx::execute(self, emp_id, |_, emp| {
            emp.set_classification(classification);
            emp.set_schedule(schedule);
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeePaymentClassificationTx<Ctx> for T where T: ChangeEmployeeTx<Ctx> {}
