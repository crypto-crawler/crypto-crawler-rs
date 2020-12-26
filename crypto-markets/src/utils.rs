pub(crate) fn calc_precision(number: f64) -> i64 {
    -number.log10() as i64
}
