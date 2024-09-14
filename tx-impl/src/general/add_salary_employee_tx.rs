use std::{cell::RefCell, rc::Rc};

use abstract_tx::{AddEmployeeTx, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{PaymentClassificationImpl, PaymentScheduleImpl};

pub trait AddSalaryEmployeeTx<Ctx>: AddEmployeeTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        AddEmployeeTx::execute(
            self,
            emp_id,
            name,
            address,
            Rc::new(RefCell::new(PaymentClassificationImpl::Salaried { salary })),
            Rc::new(RefCell::new(PaymentScheduleImpl::Monthly)),
        )
    }
}
// blanket implementation
impl<T, Ctx> AddSalaryEmployeeTx<Ctx> for T where T: AddEmployeeTx<Ctx> {}
