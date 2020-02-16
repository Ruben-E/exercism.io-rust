pub fn is_leap_year(year: u64) -> bool {
    let div_by = |n| year % n == 0;

    return div_by(4) && (!div_by(100) || div_by(400));


//    return year % 400 == 0 || (year % 100 != 0 && year % 4 == 0);
}
