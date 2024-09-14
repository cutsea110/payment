use parsec_rs::Parser;
use std::collections::VecDeque;

use mock_db::MockDb;
use mock_tx_impl::*;
use parser::{transactions, Command};
use tx_app::{Transaction, TransactionSource};

pub struct TextParserTransactionSource {
    txs: VecDeque<Box<dyn Transaction<()>>>,
}
impl TransactionSource<()> for TextParserTransactionSource {
    fn get_transaction(&mut self) -> Option<Box<dyn Transaction<()>>> {
        self.txs.pop_front()
    }
}
impl TextParserTransactionSource {
    pub fn new(db: MockDb, input: String) -> Self {
        let txs = transactions()
            .parse(&input)
            .map(|(ts, _)| {
                ts.into_iter()
                    .map(|t| to_tx(t, db.clone()))
                    .collect::<VecDeque<_>>()
            })
            .unwrap_or_default();

        Self { txs }
    }
}

fn to_tx(command: Command, db: MockDb) -> Box<dyn Transaction<()>> {
    match command {
        Command::AddSalaryEmp {
            emp_id,
            name,
            address,
            salary,
        } => Box::new(AddSalaryEmployeeTxImpl {
            db,
            emp_id,
            name,
            address,
            salary,
        }),
        Command::AddHourlyEmp {
            emp_id,
            name,
            address,
            hourly_rate,
        } => Box::new(AddHourlyEmployeeTxImpl {
            db,
            emp_id,
            name,
            address,
            hourly_rate,
        }),
        Command::AddCommissionedEmp {
            emp_id,
            name,
            address,
            salary,
            commission_rate,
        } => Box::new(AddCommissionedEmployeeTxImpl {
            db,
            emp_id,
            name,
            address,
            salary,
            commission_rate,
        }),
        Command::DelEmp { emp_id } => Box::new(DeleteEmployeeTxImpl { db, emp_id }),
        Command::TimeCard {
            emp_id,
            date,
            hours,
        } => Box::new(TimeCardTxImpl {
            db,
            emp_id,
            date,
            hours,
        }),
        Command::SalesReceipt {
            emp_id,
            date,
            amount,
        } => Box::new(SalesReceiptTxImpl {
            db,
            emp_id,
            date,
            amount,
        }),
        Command::ServiceCharge {
            member_id,
            date,
            amount,
        } => Box::new(ServiceChargeTxImpl {
            db,
            member_id,
            date,
            amount,
        }),
        Command::ChgName { emp_id, name } => {
            Box::new(ChangeEmployeeNameTxImpl { db, emp_id, name })
        }
        Command::ChgAddress { emp_id, address } => Box::new(ChangeEmployeeAddressTxImpl {
            db,
            emp_id,
            address,
        }),
        Command::ChgSalaried { emp_id, salary } => {
            Box::new(ChangeEmployeeSalariedTxImpl { db, emp_id, salary })
        }
        Command::ChgHourly {
            emp_id,
            hourly_rate,
        } => Box::new(ChangeEmployeeHourlyTxImpl {
            db,
            emp_id,
            hourly_rate,
        }),
        Command::ChgCommissioned {
            emp_id,
            salary,
            commission_rate,
        } => Box::new(ChangeEmployeeCommissionedTxImpl {
            db,
            emp_id,
            salary,
            commission_rate,
        }),
        Command::ChgHold { emp_id } => Box::new(ChangeEmployeeHoldTxImpl { db, emp_id }),
        Command::ChgDirect {
            emp_id,
            bank,
            account,
        } => Box::new(ChangeEmployeeDirectTxImpl {
            db,
            emp_id,
            bank,
            account,
        }),
        Command::ChgMail { emp_id, address } => Box::new(ChangeEmployeeMailTxImpl {
            db,
            emp_id,
            address,
        }),
        Command::ChgMember {
            emp_id,
            member_id,
            dues,
        } => Box::new(ChangeUnionMemberTxImpl {
            db,
            emp_id,
            member_id,
            dues,
        }),
        Command::ChgNoMember { emp_id } => Box::new(ChangeUnaffiliatedTxImpl { db, emp_id }),
        Command::Payday { pay_date } => Box::new(PaydayTxImpl { db, pay_date }),
    }
}
