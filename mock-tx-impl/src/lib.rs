use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use mock_db::MockDb;
use payroll_domain::{EmployeeId, MemberId};
use tx_app::Transaction;
use tx_impl::{
    affiliation::{ChangeUnaffiliatedTx, ChangeUnionMemberTx, ServiceChargeTx},
    classification::{
        ChangeEmployeeCommissionedTx, ChangeEmployeeHourlyTx, ChangeEmployeeSalariedTx,
    },
    general::{
        AddCommissionedEmployeeTx, AddHourlyEmployeeTx, AddSalaryEmployeeTx,
        ChangeEmployeeAddressTx, ChangeEmployeeNameTx, DeleteEmployeeTx, PaydayTx, SalesReceiptTx,
        TimeCardTx,
    },
    method::{ChangeEmployeeDirectTx, ChangeEmployeeHoldTx, ChangeEmployeeMailTx},
};

pub struct AddSalaryEmployeeTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
    pub address: String,
    pub salary: f32,
}
impl HavePayrollDao<()> for AddSalaryEmployeeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for AddSalaryEmployeeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        AddSalaryEmployeeTx::execute(
            self,
            self.emp_id,
            &self.name,
            &self.address,
            self.salary.clone(),
        )
        .map(|_| ())
        .run(ctx)
    }
}

pub struct AddHourlyEmployeeTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
    pub address: String,
    pub hourly_rate: f32,
}
impl HavePayrollDao<()> for AddHourlyEmployeeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for AddHourlyEmployeeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        AddHourlyEmployeeTx::execute(
            self,
            self.emp_id,
            &self.name,
            &self.address,
            self.hourly_rate.clone(),
        )
        .map(|_| ())
        .run(ctx)
    }
}

pub struct AddCommissionedEmployeeTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
    pub address: String,
    pub salary: f32,
    pub commission_rate: f32,
}
impl HavePayrollDao<()> for AddCommissionedEmployeeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for AddCommissionedEmployeeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        AddCommissionedEmployeeTx::execute(
            self,
            self.emp_id,
            &self.name,
            &self.address,
            self.salary.clone(),
            self.commission_rate.clone(),
        )
        .map(|_| ())
        .run(ctx)
    }
}

pub struct ChangeEmployeeNameTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
}
impl HavePayrollDao<()> for ChangeEmployeeNameTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeNameTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeNameTx::execute(self, self.emp_id, &self.name)
            .map(|_| ())
            .run(ctx)
    }
}

pub struct ChangeEmployeeAddressTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub address: String,
}
impl HavePayrollDao<()> for ChangeEmployeeAddressTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeAddressTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeAddressTx::execute(self, self.emp_id, &self.address)
            .map(|_| ())
            .run(ctx)
    }
}

pub struct ChangeEmployeeSalariedTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub salary: f32,
}
impl HavePayrollDao<()> for ChangeEmployeeSalariedTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeSalariedTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeSalariedTx::execute(self, self.emp_id, self.salary)
            .map(|_| ())
            .run(ctx)
    }
}

pub struct ChangeEmployeeHourlyTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub hourly_rate: f32,
}
impl HavePayrollDao<()> for ChangeEmployeeHourlyTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeHourlyTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeHourlyTx::execute(self, self.emp_id, self.hourly_rate)
            .map(|_| ())
            .run(ctx)
    }
}

pub struct ChangeEmployeeCommissionedTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub salary: f32,
    pub commission_rate: f32,
}
impl HavePayrollDao<()> for ChangeEmployeeCommissionedTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeCommissionedTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeCommissionedTx::execute(self, self.emp_id, self.salary, self.commission_rate)
            .map(|_| ())
            .run(ctx)
    }
}

pub struct ChangeEmployeeHoldTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
}
impl HavePayrollDao<()> for ChangeEmployeeHoldTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeHoldTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeHoldTx::execute(self, self.emp_id)
            .map(|_| ())
            .run(ctx)
    }
}

pub struct ChangeEmployeeMailTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub address: String,
}
impl HavePayrollDao<()> for ChangeEmployeeMailTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeMailTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeMailTx::execute(self, self.emp_id, &self.address)
            .map(|_| ())
            .run(ctx)
    }
}

pub struct ChangeEmployeeDirectTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub bank: String,
    pub account: String,
}
impl HavePayrollDao<()> for ChangeEmployeeDirectTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeEmployeeDirectTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeDirectTx::execute(self, self.emp_id, &self.bank, &self.account)
            .map(|_| ())
            .run(ctx)
    }
}

pub struct TimeCardTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub date: NaiveDate,
    pub hours: f32,
}
impl HavePayrollDao<()> for TimeCardTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for TimeCardTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        TimeCardTx::execute(self, self.emp_id, self.date, self.hours)
            .map(|_| ())
            .run(ctx)
    }
}

pub struct SalesReceiptTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub date: NaiveDate,
    pub amount: f32,
}
impl HavePayrollDao<()> for SalesReceiptTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for SalesReceiptTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        SalesReceiptTx::execute(self, self.emp_id, self.date, self.amount)
            .map(|_| ())
            .run(ctx)
    }
}

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

pub struct ChangeUnaffiliatedTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
}
impl HavePayrollDao<()> for ChangeUnaffiliatedTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ChangeUnaffiliatedTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeUnaffiliatedTx::execute(self, self.emp_id)
            .map(|_| ())
            .run(ctx)
    }
}

pub struct ServiceChargeTxImpl {
    pub db: MockDb,

    pub member_id: MemberId,
    pub date: NaiveDate,
    pub amount: f32,
}
impl HavePayrollDao<()> for ServiceChargeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for ServiceChargeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ServiceChargeTx::execute(self, self.member_id, self.date, self.amount)
            .map(|_| ())
            .run(ctx)
    }
}

pub struct DeleteEmployeeTxImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
}
impl HavePayrollDao<()> for DeleteEmployeeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.db
    }
}
impl Transaction<()> for DeleteEmployeeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        DeleteEmployeeTx::execute(self, self.emp_id)
            .map(|_| ())
            .run(ctx)
    }
}

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
