use core::cmp::Ordering;
use rug::{float::Round, ops::MulAssignRound, Float};
pub struct F(pub f64);
#[derive(Debug)]
pub enum IntIs {
    Greater,
    Lesser,
}
impl MulAssignRound<f64> for F {
    type Round = Round;
    type Ordering = Ordering;
    fn mul_assign_round(&mut self, rhs: f64, round: Round) -> Ordering {
        let mut f = Float::with_val(53, self.0);
        let dir = f.mul_assign_round(rhs, round);
        self.0 = f.to_f64();
        dir
    }
}
pub fn check1k(num: &Float) -> IntIs {
    let compared_int = if *num > 1000 {
        IntIs::Greater
    } else {
        IntIs::Lesser
    };
    compared_int
}
pub fn format_number(num:&Float) -> String {
        let compared_int = check1k(num);
        let num = num.to_integer().unwrap();
        let formatted_num = match compared_int {
            IntIs::Greater => format!("{:.4e}", num),
            _ => format!("{}", num),
        };
        formatted_num
}
