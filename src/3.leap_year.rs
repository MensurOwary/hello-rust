// next 20 leap years printer
// usual trait
trait LeapYear {
    fn is_leap_year(self) -> bool;
}

impl LeapYear for u16 {

    fn is_leap_year(self) -> bool {
        // if statement evaluated as the return value
        if self % 400 == 0 {
            true
        } else if self % 4 == 0 && self % 100 != 0 {
            true
        } else {
            false
        }
    }

}

fn leap_year_displayer () {
    let mut num: u16 = 2015;
    // while loop...standard
    while num <= 2050 {
        let result = num.is_leap_year();
        println!("{} {} a leap year", num, if result {"is"} else {"is not"});
        num = num + 1;
    }
}

fn main() {
    leap_year_displayer();
}
