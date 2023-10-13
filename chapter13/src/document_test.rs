use super::target::Guest;

/// ## Example
/// ```
/// use chapter13::document_test::calc_fee_case01;
///
/// let res = calc_fee_case01();
/// assert_eq!(res, 500);
/// ```
pub fn calc_fee_case01() -> u32 {
    let guest = Guest::new(10, false);

    guest.clone().calc_fee().unwrap()
}
