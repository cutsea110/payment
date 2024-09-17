use mock_db::MockDb;
use payroll_domain::{EmployeeId, MemberId};
use tx_app::Transaction;
use tx_factory::TransactionFactory;

#[derive(Debug)]
pub struct TransactionFactoryImpl {
    db: MockDb,
}
impl TransactionFactoryImpl {
    pub fn new(db: MockDb) -> Self {
        Self { db }
    }
}
impl TransactionFactory<()> for TransactionFactoryImpl {
    fn mk_add_salary_employee_tx(
        &self,
        emp_id: EmployeeId,
        name: String,
        address: String,
        salary: f32,
    ) -> Box<dyn Transaction<()>> {
        Box::new(crate::add_salary_employee_tx::AddSalaryEmployeeTxImpl {
            db: self.db.clone(),
            emp_id,
            name,
            address,
            salary,
        })
    }
    fn mk_add_hourly_employee_tx(
        &self,
        emp_id: EmployeeId,
        name: String,
        address: String,
        hourly_rate: f32,
    ) -> Box<dyn Transaction<()>> {
        Box::new(crate::add_hourly_employee_tx::AddHourlyEmployeeTxImpl {
            db: self.db.clone(),
            emp_id,
            name,
            address,
            hourly_rate,
        })
    }
    fn mk_add_commissioned_employee_tx(
        &self,
        emp_id: EmployeeId,
        name: String,
        address: String,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction<()>> {
        Box::new(
            crate::add_commissioned_employee_tx::AddCommissionedEmployeeTxImpl {
                db: self.db.clone(),
                emp_id,
                name,
                address,
                salary,
                commission_rate,
            },
        )
    }
    fn mk_delete_employee_tx(&self, emp_id: EmployeeId) -> Box<dyn Transaction<()>> {
        Box::new(crate::delete_employee_tx::DeleteEmployeeTxImpl {
            db: self.db.clone(),
            emp_id,
        })
    }
    fn mk_timecard_tx(
        &self,
        emp_id: EmployeeId,
        date: chrono::NaiveDate,
        hours: f32,
    ) -> Box<dyn Transaction<()>> {
        Box::new(crate::timecard_tx::TimeCardTxImpl {
            db: self.db.clone(),
            emp_id,
            date,
            hours,
        })
    }
    fn mk_sales_receipt_tx(
        &self,
        emp_id: EmployeeId,
        date: chrono::NaiveDate,
        amount: f32,
    ) -> Box<dyn Transaction<()>> {
        Box::new(crate::sales_receipt_tx::SalesReceiptTxImpl {
            db: self.db.clone(),
            emp_id,
            date,
            amount,
        })
    }
    fn mk_change_name_tx(&self, emp_id: EmployeeId, name: String) -> Box<dyn Transaction<()>> {
        Box::new(crate::change_employee_name_tx::ChangeEmployeeNameTxImpl {
            db: self.db.clone(),
            emp_id,
            name,
        })
    }
    fn mk_change_address_tx(
        &self,
        emp_id: EmployeeId,
        address: String,
    ) -> Box<dyn Transaction<()>> {
        Box::new(
            crate::change_employee_address_tx::ChangeEmployeeAddressTxImpl {
                db: self.db.clone(),
                emp_id,
                address: address.to_string(),
            },
        )
    }
    fn mk_change_salaried_tx(&self, emp_id: EmployeeId, salary: f32) -> Box<dyn Transaction<()>> {
        Box::new(
            crate::change_employee_salaried_tx::ChangeEmployeeSalariedTxImpl {
                db: self.db.clone(),
                emp_id,
                salary,
            },
        )
    }
    fn mk_change_hourly_tx(
        &self,
        emp_id: EmployeeId,
        hourly_rate: f32,
    ) -> Box<dyn Transaction<()>> {
        Box::new(
            crate::change_employee_hourly_tx::ChangeEmployeeHourlyTxImpl {
                db: self.db.clone(),
                emp_id,
                hourly_rate,
            },
        )
    }
    fn mk_change_commissioned_tx(
        &self,
        emp_id: EmployeeId,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction<()>> {
        Box::new(
            crate::change_employee_commissioned_tx::ChangeEmployeeCommissionedTxImpl {
                db: self.db.clone(),
                emp_id,
                salary,
                commission_rate,
            },
        )
    }
    fn mk_change_direct_tx(
        &self,
        emp_id: EmployeeId,
        bank: String,
        account: String,
    ) -> Box<dyn Transaction<()>> {
        Box::new(
            crate::change_employee_direct_tx::ChangeEmployeeDirectTxImpl {
                db: self.db.clone(),
                emp_id,
                bank,
                account,
            },
        )
    }
    fn mk_change_mail_tx(&self, emp_id: EmployeeId, address: String) -> Box<dyn Transaction<()>> {
        Box::new(crate::change_employee_mail_tx::ChangeEmployeeMailTxImpl {
            db: self.db.clone(),
            emp_id,
            address,
        })
    }
    fn mk_change_hold_tx(&self, emp_id: EmployeeId) -> Box<dyn Transaction<()>> {
        Box::new(crate::change_employee_hold_tx::ChangeEmployeeHoldTxImpl {
            db: self.db.clone(),
            emp_id,
        })
    }
    fn mk_change_union_member_tx(
        &self,
        emp_id: EmployeeId,
        member_id: MemberId,
        dues: f32,
    ) -> Box<dyn Transaction<()>> {
        Box::new(crate::change_union_member_tx::ChangeUnionMemberTxImpl {
            db: self.db.clone(),
            emp_id,
            member_id,
            dues,
        })
    }
    fn mk_change_unaffiliated_tx(&self, emp_id: EmployeeId) -> Box<dyn Transaction<()>> {
        Box::new(crate::change_unaffiliated_tx::ChangeUnaffiliatedTxImpl {
            db: self.db.clone(),
            emp_id,
        })
    }
    fn mk_service_charge_tx(
        &self,
        member_id: MemberId,
        date: chrono::prelude::NaiveDate,
        amount: f32,
    ) -> Box<dyn Transaction<()>> {
        Box::new(crate::service_charge_tx::ServiceChargeTxImpl {
            db: self.db.clone(),
            member_id,
            date,
            amount,
        })
    }
    fn mk_payday_tx(&self, pay_date: chrono::NaiveDate) -> Box<dyn Transaction<()>> {
        Box::new(crate::payday_tx::PaydayTxImpl {
            db: self.db.clone(),
            pay_date,
        })
    }
}
