use payroll_domain::{Paycheck, PaymentMethod};

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethodImpl {
    Hold,
    Mail { address: String },
    Direct { bank: String, account: String },
}
impl PaymentMethod for PaymentMethodImpl {
    fn pay(&self, pc: &Paycheck) {
        match self {
            PaymentMethodImpl::Hold => {
                println!("Hold the check: {:#?}", pc);
            }
            PaymentMethodImpl::Mail { address } => {
                println!("Send check to {} by Mail: {:#?}", address, pc);
            }
            PaymentMethodImpl::Direct { bank, account } => {
                println!(
                    "Direct deposit ${} to {} at {}: {:#?}",
                    pc.get_net_pay(),
                    account,
                    bank,
                    pc
                );
            }
        }
    }
}
