use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use crate::error::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use payroll_domain::{Employee, EmployeeId, PaymentClassification, PaymentSchedule};
use payroll_impl::{AffiliationImpl, PaymentMethodImpl};

pub trait AddEmployeeTx<Ctx>: HavePayrollDao<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        classification: Rc<RefCell<dyn PaymentClassification>>,
        schedule: Rc<RefCell<dyn PaymentSchedule>>,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        let emp = Employee::new(
            emp_id,
            name,
            address,
            classification,
            schedule,
            Rc::new(RefCell::new(PaymentMethodImpl::Hold)),
            Rc::new(RefCell::new(AffiliationImpl::Unaffiliated)),
        );
        self.dao()
            .insert(emp)
            .map_err(UsecaseError::RegisterEmployeeFailed)
    }
}
// blanket implementation
impl<T, Ctx> AddEmployeeTx<Ctx> for T where T: HavePayrollDao<Ctx> {}
