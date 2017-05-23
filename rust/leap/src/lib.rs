pub fn is_leap_year(year: i64) -> bool {
    let evenly_divisible_by_4 = 0 == (year % 4);
    let evenly_divisible_by_100 = 0 == (year % 100);
    let evenly_divisible_by_400 = 0 == (year % 400);

    evenly_divisible_by_4 && !(evenly_divisible_by_100 && !evenly_divisible_by_400)
}
