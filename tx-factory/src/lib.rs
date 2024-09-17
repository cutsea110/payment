use chrono::NaiveDate;

use payroll_domain::{EmployeeId, MemberId};
use tx_app::Transaction;

pub trait TransactionFactory<Ctx> {
    fn mk_add_salary_employee_tx(
        &self,
        emp_id: EmployeeId,
        name: String,
        address: String,
        salary: f32,
    ) -> Box<dyn Transaction<Ctx>>;
    fn mk_add_hourly_employee_tx(
        &self,
        emp_id: EmployeeId,
        name: String,
        address: String,
        hourly_rate: f32,
    ) -> Box<dyn Transaction<Ctx>>;
    fn mk_add_commissioned_employee_tx(
        &self,
        emp_id: EmployeeId,
        name: String,
        address: String,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction<Ctx>>;
    fn mk_delete_employee_tx(&self, emp_id: EmployeeId) -> Box<dyn Transaction<Ctx>>;
    fn mk_timecard_tx(
        &self,
        emp_id: EmployeeId,
        date: NaiveDate,
        hours: f32,
    ) -> Box<dyn Transaction<Ctx>>;
    fn mk_sales_receipt_tx(
        &self,
        emp_id: EmployeeId,
        date: NaiveDate,
        amount: f32,
    ) -> Box<dyn Transaction<Ctx>>;
    fn mk_change_name_tx(&self, emp_id: EmployeeId, name: String) -> Box<dyn Transaction<Ctx>>;
    fn mk_change_address_tx(
        &self,
        emp_id: EmployeeId,
        address: String,
    ) -> Box<dyn Transaction<Ctx>>;
    fn mk_change_salaried_tx(&self, emp_id: EmployeeId, salary: f32) -> Box<dyn Transaction<Ctx>>;
    fn mk_change_hourly_tx(
        &self,
        emp_id: EmployeeId,
        hourly_rate: f32,
    ) -> Box<dyn Transaction<Ctx>>;
    fn mk_change_commissioned_tx(
        &self,
        emp_id: EmployeeId,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction<Ctx>>;
    fn mk_change_direct_tx(
        &self,
        emp_id: EmployeeId,
        bank: String,
        account: String,
    ) -> Box<dyn Transaction<Ctx>>;
    fn mk_change_mail_tx(&self, emp_id: EmployeeId, address: String) -> Box<dyn Transaction<Ctx>>;
    fn mk_change_hold_tx(&self, emp_id: EmployeeId) -> Box<dyn Transaction<Ctx>>;
    fn mk_change_union_member_tx(
        &self,
        emp_id: EmployeeId,
        member_id: MemberId,
        dues: f32,
    ) -> Box<dyn Transaction<Ctx>>;
    fn mk_change_unaffiliated_tx(&self, emp_id: EmployeeId) -> Box<dyn Transaction<Ctx>>;
    fn mk_service_charge_tx(
        &self,
        member_id: MemberId,
        date: NaiveDate,
        amount: f32,
    ) -> Box<dyn Transaction<Ctx>>;
    fn mk_payday_tx(&self, pay_date: NaiveDate) -> Box<dyn Transaction<Ctx>>;
}
