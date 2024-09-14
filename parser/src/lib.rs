use chrono::NaiveDate;
use parsec_rs::{char, float32, int32, keyword, pred, spaces, string, uint32, Parser};

use payroll_domain::EmployeeId;

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
pub fn transactions() -> impl Parser<Item = Vec<Command>> {
    transaction().many0()
}
pub fn transaction() -> impl Parser<Item = Command> {
    go_through().skip(
        add_salary_emp()
            .or(add_hourly_emp())
            .or(add_commissioned_emp())
            .or(del_emp())
            .or(time_card())
            .or(sales_receipt())
            .or(service_charge())
            .or(chg_name())
            .or(chg_address())
            .or(chg_hourly())
            .or(chg_salaried())
            .or(chg_commissioned())
            .or(chg_hold())
            .or(chg_direct())
            .or(chg_mail())
            .or(chg_member())
            .or(chg_no_member())
            .or(payday()),
    )
}
#[cfg(test)]
mod test_transaction {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test_go_through() {
        let input = "";
        let result = go_through().parse(input);
        assert_eq!(result, Ok(((), "")));

        let input = "Code";
        let result = go_through().parse(input);
        assert_eq!(result, Ok(((), "Code")));

        let input = "# comment\nCode";
        let result = go_through().parse(input);
        assert_eq!(result, Ok(((), "Code")));

        let input = "# comment\n#\n# comment\nCode";
        let result = go_through().parse(input);
        assert_eq!(result, Ok(((), "Code")));

        let input = " \t\n# comment\n#\nCode";
        let result = go_through().parse(input);
        assert_eq!(result, Ok(((), "Code")));

        let input = " \t\n# comment\n#\n \tCode";
        let result = go_through().parse(input);
        assert_eq!(result, Ok(((), "Code")));
    }

    #[test]
    fn test_add_salary_emp() {
        let input = r#"AddEmp 42 "Bob" "Home" S 1000.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::AddSalaryEmp {
                    emp_id: 42,
                    name: "Bob".to_string(),
                    address: "Home".to_string(),
                    salary: 1000.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_add_hourly_emp() {
        let input = r#"AddEmp 42 "Bob" "Home" H 1000.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::AddHourlyEmp {
                    emp_id: 42,
                    name: "Bob".to_string(),
                    address: "Home".to_string(),
                    hourly_rate: 1000.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_add_commissioned_emp() {
        let input = r#"AddEmp 42 "Bob" "Home" C 1000.0 0.1"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::AddCommissionedEmp {
                    emp_id: 42,
                    name: "Bob".to_string(),
                    address: "Home".to_string(),
                    salary: 1000.0,
                    commission_rate: 0.1
                },
                ""
            ))
        );
    }
    #[test]
    fn test_del_emp() {
        let input = r#"DelEmp 42"#;
        let result = transaction().parse(input);
        assert_eq!(result, Ok((Command::DelEmp { emp_id: 42 }, "")));
    }
    #[test]
    fn test_time_card() {
        let input = r#"TimeCard 42 2021-01-01 8.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::TimeCard {
                    emp_id: 42,
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    hours: 8.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_sales_receipt() {
        let input = r#"SalesReceipt 42 2021-01-01 1000.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::SalesReceipt {
                    emp_id: 42,
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    amount: 1000.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_service_charge() {
        let input = r#"ServiceCharge 42 2021-01-01 1000.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ServiceCharge {
                    member_id: 42,
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    amount: 1000.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_name() {
        let input = r#"ChgEmp 42 Name "Bob""#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgName {
                    emp_id: 42,
                    name: "Bob".to_string()
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_address() {
        let input = r#"ChgEmp 42 Address "123 Wall St.""#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgAddress {
                    emp_id: 42,
                    address: "123 Wall St.".to_string()
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_hourly() {
        let input = r#"ChgEmp 42 Hourly 1000.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgHourly {
                    emp_id: 42,
                    hourly_rate: 1000.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_salaried() {
        let input = r#"ChgEmp 42 Salaried 1000.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgSalaried {
                    emp_id: 42,
                    salary: 1000.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_commissioned() {
        let input = r#"ChgEmp 42 Commissioned 1000.0 0.1"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgCommissioned {
                    emp_id: 42,
                    salary: 1000.0,
                    commission_rate: 0.1
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_hold() {
        let input = r#"ChgEmp 42 Hold"#;
        let result = transaction().parse(input);
        assert_eq!(result, Ok((Command::ChgHold { emp_id: 42 }, "")));
    }
    #[test]
    fn test_chg_direct() {
        let input = r#"ChgEmp 42 Direct "mufg" "1234567""#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgDirect {
                    emp_id: 42,
                    bank: "mufg".to_string(),
                    account: "1234567".to_string()
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_mail() {
        let input = r#"ChgEmp 42 Mail "bob@gmail.com""#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgMail {
                    emp_id: 42,
                    address: "bob@gmail.com".to_string()
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_member() {
        let input = r#"ChgEmp 42 Member 7234 Dues 9.45"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgMember {
                    emp_id: 42,
                    member_id: 7234,
                    dues: 9.45,
                },
                "",
            ))
        );
    }
    #[test]
    fn test_no_member() {
        let input = r#"ChgEmp 42 NoMember"#;
        let result = transaction().parse(input);
        assert_eq!(result, Ok((Command::ChgNoMember { emp_id: 42 }, "")));
    }
}

fn go_through() -> impl Parser<Item = ()> {
    let comment = char('#').skip(pred(|c| c != '\n').many0().with(char('\n')));
    let space_comment = spaces().skip(comment).map(|_| ());
    let ignore = space_comment.many1().map(|_| ()).or(spaces().map(|_| ()));

    spaces().skip(ignore).skip(spaces()).map(|_| ())
}

fn add_salary_emp() -> impl Parser<Item = Command> {
    let prefix = keyword("AddEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let name = string().with(spaces());
    let address = string().with(spaces());
    let monthly_rate = char('S').skip(spaces()).skip(float32());

    prefix
        .skip(emp_id)
        .join(name)
        .join(address)
        .join(monthly_rate)
        .map(
            |(((emp_id, name), address), salary)| Command::AddSalaryEmp {
                emp_id,
                name,
                address,
                salary,
            },
        )
}
#[cfg(test)]
mod test_add_salary_emp {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"AddEmp 1 "Bob" "Home" S 1000.0"#;
        let result = add_salary_emp().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::AddSalaryEmp {
                    emp_id: 1,
                    name: "Bob".to_string(),
                    address: "Home".to_string(),
                    salary: 1000.0
                },
                ""
            ))
        );
    }
}

fn add_hourly_emp() -> impl Parser<Item = Command> {
    let prefix = keyword("AddEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let name = string().with(spaces());
    let address = string().with(spaces());
    let hourly_rate = char('H').skip(spaces()).skip(float32());

    prefix
        .skip(emp_id)
        .join(name)
        .join(address)
        .join(hourly_rate)
        .map(
            |(((emp_id, name), address), hourly_rate)| Command::AddHourlyEmp {
                emp_id,
                name,
                address,
                hourly_rate,
            },
        )
}
#[cfg(test)]
mod test_add_hourly_emp {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"AddEmp 1 "Bob" "Home" H 1000.0"#;
        let result = add_hourly_emp().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::AddHourlyEmp {
                    emp_id: 1,
                    name: "Bob".to_string(),
                    address: "Home".to_string(),
                    hourly_rate: 1000.0
                },
                ""
            ))
        );
    }
}

fn add_commissioned_emp() -> impl Parser<Item = Command> {
    let prefix = keyword("AddEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let name = string().with(spaces());
    let address = string().with(spaces());
    let salary = char('C').skip(spaces()).skip(float32()).with(spaces());
    let commission_rate = float32();

    prefix
        .skip(emp_id)
        .join(name)
        .join(address)
        .join(salary)
        .join(commission_rate)
        .map(
            |((((emp_id, name), address), salary), commission_rate)| Command::AddCommissionedEmp {
                emp_id,
                name,
                address,
                salary,
                commission_rate,
            },
        )
}
#[cfg(test)]
mod test_add_commissioned_emp {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"AddEmp 1 "Bob" "Home" C 1000.0 0.1"#;
        let result = add_commissioned_emp().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::AddCommissionedEmp {
                    emp_id: 1,
                    name: "Bob".to_string(),
                    address: "Home".to_string(),
                    salary: 1000.0,
                    commission_rate: 0.1
                },
                ""
            ))
        );
    }
}

fn del_emp() -> impl Parser<Item = Command> {
    let prefix = keyword("DelEmp").skip(spaces());
    let emp_id = uint32();

    prefix.skip(emp_id).map(|emp_id| Command::DelEmp { emp_id })
}
#[cfg(test)]
mod test_del_emp {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"DelEmp 1"#;
        let result = del_emp().parse(input);
        assert_eq!(result, Ok((Command::DelEmp { emp_id: 1 }, "")));
    }
}

fn date() -> impl Parser<Item = NaiveDate> {
    let year = int32().with(char('-'));
    let month = uint32().with(char('-'));
    let day = uint32();

    year.join(month)
        .join(day)
        .map(|((y, m), d)| NaiveDate::from_ymd_opt(y as i32, m as u32, d as u32).expect("date"))
}
#[cfg(test)]
mod test_date {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = "2021-01-01";
        let result = date().parse(input);
        assert_eq!(
            result,
            Ok((NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), ""))
        );
    }
}

fn time_card() -> impl Parser<Item = Command> {
    let prefix = keyword("TimeCard").skip(spaces());
    let emp_id = uint32().with(spaces());
    let date = date().with(spaces());
    let hours = float32();

    prefix
        .skip(emp_id)
        .join(date)
        .join(hours)
        .map(|((emp_id, date), hours)| Command::TimeCard {
            emp_id,
            date,
            hours,
        })
}
#[cfg(test)]
mod test_time_card {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"TimeCard 1 2021-01-01 8.0"#;
        let result = time_card().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::TimeCard {
                    emp_id: 1,
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    hours: 8.0
                },
                ""
            ))
        );
    }
}

fn sales_receipt() -> impl Parser<Item = Command> {
    let prefix = keyword("SalesReceipt").skip(spaces());
    let emp_id = uint32().with(spaces());
    let date = date().with(spaces());
    let amount = float32();

    prefix
        .skip(emp_id)
        .join(date)
        .join(amount)
        .map(|((emp_id, date), amount)| Command::SalesReceipt {
            emp_id,
            date,
            amount,
        })
}
#[cfg(test)]
mod test_sales_receipt {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"SalesReceipt 1 2021-01-01 1000.0"#;
        let result = sales_receipt().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::SalesReceipt {
                    emp_id: 1,
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    amount: 1000.0
                },
                ""
            ))
        );
    }
}

fn service_charge() -> impl Parser<Item = Command> {
    let prefix = keyword("ServiceCharge").skip(spaces());
    let member_id = uint32().with(spaces());
    let date = date().with(spaces());
    let amount = float32();

    prefix
        .skip(member_id)
        .join(date)
        .join(amount)
        .map(|((member_id, date), amount)| Command::ServiceCharge {
            member_id,
            date,
            amount,
        })
}
#[cfg(test)]
mod test_service_charge {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ServiceCharge 1 2021-01-01 1000.0"#;
        let result = service_charge().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ServiceCharge {
                    member_id: 1,
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    amount: 1000.0
                },
                ""
            ))
        );
    }
}

fn chg_name() -> impl Parser<Item = Command> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let name = keyword("Name").skip(spaces()).skip(string());

    prefix
        .skip(emp_id)
        .join(name)
        .map(|(emp_id, name)| Command::ChgName { emp_id, name })
}
#[cfg(test)]
mod test_chg_name {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Name "Bob""#;
        let result = chg_name().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgName {
                    emp_id: 1,
                    name: "Bob".to_string()
                },
                ""
            ))
        );
    }
}

fn chg_address() -> impl Parser<Item = Command> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let address = keyword("Address").skip(spaces()).skip(string());

    prefix
        .skip(emp_id)
        .join(address)
        .map(|(emp_id, address)| Command::ChgAddress { emp_id, address })
}
#[cfg(test)]
mod test_chg_address {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Address "123 Main St""#;
        let result = chg_address().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgAddress {
                    emp_id: 1,
                    address: "123 Main St".to_string()
                },
                ""
            ))
        );
    }
}

fn chg_hourly() -> impl Parser<Item = Command> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let hourly_rate = keyword("Hourly").skip(spaces()).skip(float32());

    prefix
        .skip(emp_id)
        .join(hourly_rate)
        .map(|(emp_id, hourly_rate)| Command::ChgHourly {
            emp_id,
            hourly_rate,
        })
}
#[cfg(test)]
mod test_chg_hourly {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Hourly 13.78"#;
        let result = chg_hourly().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgHourly {
                    emp_id: 1,
                    hourly_rate: 13.78
                },
                ""
            ))
        );
    }
}

fn chg_salaried() -> impl Parser<Item = Command> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let salaried = keyword("Salaried").skip(spaces()).skip(float32());

    prefix
        .skip(emp_id)
        .join(salaried)
        .map(|(emp_id, salary)| Command::ChgSalaried { emp_id, salary })
}
#[cfg(test)]
mod test_chg_salaried {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Salaried 1023.456"#;
        let result = chg_salaried().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgSalaried {
                    emp_id: 1,
                    salary: 1023.456
                },
                ""
            ))
        );
    }
}

fn chg_commissioned() -> impl Parser<Item = Command> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let salary = keyword("Commissioned")
        .skip(spaces())
        .skip(float32())
        .with(spaces());
    let commission_rate = float32();

    prefix.skip(emp_id).join(salary).join(commission_rate).map(
        |((emp_id, salary), commission_rate)| Command::ChgCommissioned {
            emp_id,
            salary,
            commission_rate,
        },
    )
}
#[cfg(test)]
mod test_chg_commissioned {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Commissioned 1018.91 0.19"#;
        let result = chg_commissioned().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgCommissioned {
                    emp_id: 1,
                    salary: 1018.91,
                    commission_rate: 0.19
                },
                ""
            ))
        );
    }
}

fn chg_hold() -> impl Parser<Item = Command> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let hold = keyword("Hold");

    prefix
        .skip(emp_id)
        .with(hold)
        .map(|emp_id| Command::ChgHold { emp_id })
}
#[cfg(test)]
mod test_chg_hold {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Hold"#;
        let result = chg_hold().parse(input);
        assert_eq!(result, Ok((Command::ChgHold { emp_id: 1 }, "")));
    }
}

fn chg_direct() -> impl Parser<Item = Command> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let bank = keyword("Direct")
        .skip(spaces())
        .skip(string())
        .with(spaces());
    let account = string();

    prefix
        .skip(emp_id)
        .join(bank)
        .join(account)
        .map(|((emp_id, bank), account)| Command::ChgDirect {
            emp_id,
            bank,
            account,
        })
}
#[cfg(test)]
mod test_chg_direct {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Direct "Bank" "Account""#;
        let result = chg_direct().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgDirect {
                    emp_id: 1,
                    bank: "Bank".to_string(),
                    account: "Account".to_string()
                },
                ""
            ))
        );
    }
}

fn chg_mail() -> impl Parser<Item = Command> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let address = keyword("Mail").skip(spaces()).skip(string());

    prefix
        .skip(emp_id)
        .join(address)
        .map(|(emp_id, address)| Command::ChgMail { emp_id, address })
}
#[cfg(test)]
mod test_chg_mail {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Mail "bob@gmail.com""#;
        let result = chg_mail().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgMail {
                    emp_id: 1,
                    address: "bob@gmail.com".to_string()
                },
                ""
            ))
        );
    }
}

fn chg_member() -> impl Parser<Item = Command> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let member_id = keyword("Member")
        .skip(spaces())
        .skip(uint32())
        .with(spaces());
    let dues = keyword("Dues").skip(spaces()).skip(float32());

    prefix
        .skip(emp_id)
        .join(member_id)
        .join(dues)
        .map(|((emp_id, member_id), dues)| Command::ChgMember {
            emp_id,
            member_id,
            dues,
        })
}
#[cfg(test)]
mod test_chg_member {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Member 2 Dues 100.0"#;
        let result = chg_member().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::ChgMember {
                    emp_id: 1,
                    member_id: 2,
                    dues: 100.0
                },
                ""
            ))
        );
    }
}

fn chg_no_member() -> impl Parser<Item = Command> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = uint32().with(spaces());
    let no_member = keyword("NoMember");

    prefix
        .skip(emp_id)
        .with(no_member)
        .map(|emp_id| Command::ChgNoMember { emp_id })
}
#[cfg(test)]
mod test_chg_no_member {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 NoMember"#;
        let result = chg_no_member().parse(input);
        assert_eq!(result, Ok((Command::ChgNoMember { emp_id: 1 }, "")));
    }
}

fn payday() -> impl Parser<Item = Command> {
    let prefix = keyword("Payday").skip(spaces());
    let date = date();

    prefix
        .skip(date)
        .map(|pay_date| Command::Payday { pay_date })
}
#[cfg(test)]
mod test_payday {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"Payday 2021-01-01"#;
        let result = payday().parse(input);
        assert_eq!(
            result,
            Ok((
                Command::Payday {
                    pay_date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()
                },
                ""
            ))
        );
    }
}
