use chrono::NaiveDate;

use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_factory::TransactionFactory;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    AddSalaryEmp {
        emp_id: EmployeeId,
        name: String,
        address: String,
        salary: f32,
    },
    AddHourlyEmp {
        emp_id: EmployeeId,
        name: String,
        address: String,
        hourly_rate: f32,
    },
    AddCommissionedEmp {
        emp_id: EmployeeId,
        name: String,
        address: String,
        salary: f32,
        commission_rate: f32,
    },
    DelEmp {
        emp_id: EmployeeId,
    },
    TimeCard {
        emp_id: EmployeeId,
        date: NaiveDate,
        hours: f32,
    },
    SalesReceipt {
        emp_id: EmployeeId,
        date: NaiveDate,
        amount: f32,
    },
    ServiceCharge {
        member_id: EmployeeId,
        date: NaiveDate,
        amount: f32,
    },
    ChgName {
        emp_id: EmployeeId,
        name: String,
    },
    ChgAddress {
        emp_id: EmployeeId,
        address: String,
    },
    ChgHourly {
        emp_id: EmployeeId,
        hourly_rate: f32,
    },
    ChgSalaried {
        emp_id: EmployeeId,
        salary: f32,
    },
    ChgCommissioned {
        emp_id: EmployeeId,
        salary: f32,
        commission_rate: f32,
    },
    ChgHold {
        emp_id: EmployeeId,
    },
    ChgDirect {
        emp_id: EmployeeId,
        bank: String,
        account: String,
    },
    ChgMail {
        emp_id: EmployeeId,
        address: String,
    },
    ChgMember {
        emp_id: EmployeeId,
        member_id: EmployeeId,
        dues: f32,
    },
    ChgNoMember {
        emp_id: EmployeeId,
    },
    Payday {
        pay_date: NaiveDate,
    },
}
impl Command {
    pub fn convert<Ctx>(
        self,
        tx_factory: &impl TransactionFactory<Ctx>,
    ) -> Box<dyn Transaction<Ctx>> {
        match self {
            Command::AddSalaryEmp {
                emp_id,
                name,
                address,
                salary,
            } => tx_factory.mk_add_salary_employee_tx(emp_id, name, address, salary),
            Command::AddHourlyEmp {
                emp_id,
                name,
                address,
                hourly_rate,
            } => tx_factory.mk_add_hourly_employee_tx(emp_id, name, address, hourly_rate),
            Command::AddCommissionedEmp {
                emp_id,
                name,
                address,
                salary,
                commission_rate,
            } => tx_factory.mk_add_commissioned_employee_tx(
                emp_id,
                name,
                address,
                salary,
                commission_rate,
            ),
            Command::DelEmp { emp_id } => tx_factory.mk_delete_employee_tx(emp_id),
            Command::TimeCard {
                emp_id,
                date,
                hours,
            } => tx_factory.mk_timecard_tx(emp_id, date, hours),
            Command::SalesReceipt {
                emp_id,
                date,
                amount,
            } => tx_factory.mk_sales_receipt_tx(emp_id, date, amount),
            Command::ServiceCharge {
                member_id,
                date,
                amount,
            } => tx_factory.mk_service_charge_tx(member_id, date, amount),
            Command::ChgName { emp_id, name } => tx_factory.mk_change_name_tx(emp_id, name),
            Command::ChgAddress { emp_id, address } => {
                tx_factory.mk_change_address_tx(emp_id, address)
            }
            Command::ChgSalaried { emp_id, salary } => {
                tx_factory.mk_change_salaried_tx(emp_id, salary)
            }
            Command::ChgHourly {
                emp_id,
                hourly_rate,
            } => tx_factory.mk_change_hourly_tx(emp_id, hourly_rate),
            Command::ChgCommissioned {
                emp_id,
                salary,
                commission_rate,
            } => tx_factory.mk_change_commissioned_tx(emp_id, salary, commission_rate),
            Command::ChgHold { emp_id } => tx_factory.mk_change_hold_tx(emp_id),
            Command::ChgDirect {
                emp_id,
                bank,
                account,
            } => tx_factory.mk_change_direct_tx(emp_id, bank, account),
            Command::ChgMail { emp_id, address } => tx_factory.mk_change_mail_tx(emp_id, address),
            Command::ChgMember {
                emp_id,
                member_id,
                dues,
            } => tx_factory.mk_change_union_member_tx(emp_id, member_id, dues),
            Command::ChgNoMember { emp_id } => tx_factory.mk_change_unaffiliated_tx(emp_id),
            Command::Payday { pay_date } => tx_factory.mk_payday_tx(pay_date),
        }
    }
}
