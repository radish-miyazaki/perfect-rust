#![allow(dead_code)]

use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum SampleError {
    Msg(String)
}

impl Display for SampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleError::Msg(msg) => write!(f, "{}", msg)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Guest {
    age: u32,
    campaign: bool,
}

impl Display for Guest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Age: {}, Campaign: {}", self.age, self.campaign)
    }
}

impl Guest {
    pub fn new(age: u32, campaign: bool) -> Self {
        Self { age, campaign }
    }

    /// ### Description
    /// This method calculates the fee for the guest.
    ///
    /// ### Examples
    /// ```
    /// use chapter13::target::Guest;
    ///
    /// let guest = Guest::new(10, true);
    /// let res = guest.calc_fee().unwrap();
    /// assert_eq!(res, 450);
    /// ```
    pub fn calc_fee(self) -> Result<u32, SampleError> {
        let fee = match self.age {
            0..=4 => 0,
            5..=12 => 500,
            13..=17 => 700,
            18..=64 => 1000,
            65..=120 => 600,
            _ => return Err(SampleError::Msg("Invalid age value".to_string()))
        };

        Ok(self.calc_campaign_fee(fee))
    }

    fn calc_campaign_fee(&self, fee: u32) -> u32 {
        if self.campaign && fee != 0 {
            return fee * 90 / 100;
        }

        fee
    }
}

#[cfg(test)]
mod tests {
    use crate::target::{Guest, SampleError};

    #[test]
    fn calc_fee_test01() {
        let guest = Guest::new(10, false);
        let res = guest.clone().calc_fee().unwrap();
        assert_eq!(res, 500, "{}", guest);
    }

    #[test]
    fn calc_fee_test02() {
        let guest = Guest::new(10, true);
        let res = guest.clone().calc_fee().unwrap();
        assert_eq!(res, 450, "{}", guest);
    }

    #[test]
    fn calc_fee_test03() {
        let guest = Guest::new(18, false);
        let res = guest.clone().calc_fee().unwrap();
        assert_ne!(700, res, "{}", guest);
    }

    #[test]
    fn calc_fee_test04() {
        let guest = Guest::new(18, true);
        let res = guest.clone().calc_fee().unwrap();
        assert_ne!(630, res, "{}", guest);
    }

    #[test]
    fn calc_fee_case_wrong_age() {
        let guest = Guest::new(125, false);
        let res = guest.clone().calc_fee().unwrap_err();
        let expected = SampleError::Msg(String::from("Invalid age value"));
        assert_eq!(expected, res, "{}", guest);
    }

    #[test]
    fn calc_campaign_fee_case_01() {
        let guest = Guest::new(0, true);
        let res = guest.calc_campaign_fee(1000);
        assert_eq!(900, res);
    }

    #[test]
    #[should_panic]
    fn calc_fee_case_should_panic() {
        let guest = Guest::new(125, false);
        match guest.calc_fee() {
            Ok(res) => assert_ne!(700, res),
            Err(_) => panic!()
        };
    }

    #[test]
    fn use_debug() {
        let guest = Guest::new(0, true);
        dbg!(&guest);
        let result = guest.calc_campaign_fee(1000);
        assert_eq!(result, 900);
    }
}
