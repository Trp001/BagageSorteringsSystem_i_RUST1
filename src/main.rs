pub mod sorteringssystem;
pub mod check_in_counter;
pub mod reservationssystem;
pub mod flyveplan;

pub mod test;

use flyveplan::flyveplan;
use reservationssystem::reservationsssystem;
use sorteringssystem::{sorteringssystem};
use check_in_counter::check_in_counter;
use test::test;



fn main() {
    (sorteringssystem, check_in_counter, reservationssystem, flyveplan, test);

    #[allow(dead_code)]
    #[rustc_main]

}


