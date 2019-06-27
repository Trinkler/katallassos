use super::*;

/// Business Day Shift Convention: it defines if the schedule times are supposed to fall on only
/// business days or not. It shifts an input time according to the desired rule. See section 4.4 of
/// the ACTUS paper for details.
pub fn business_day_shift(
    mut date: UncheckedTime,
    business_day_convention: BusinessDayConvention,
) -> UncheckedTime {
    match business_day_convention {
        BusinessDayConvention::NULL => (),
        // BusinessDayConvention::SCF => {}
        // BusinessDayConvention::SCMF => {}
        // BusinessDayConvention::CSF => {}
        // BusinessDayConvention::CSMF => {}
        // BusinessDayConvention::SCP => {}
        // BusinessDayConvention::SCMP => {}
        // BusinessDayConvention::CSP => {}
        // BusinessDayConvention::CSMP => {}
    }
    date
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn business_day_shift_works() {
        let mut t = time::UncheckedTime {
            year: 2004,
            month: 2,
            day: 31,
            hour: 00,
            minute: 00,
            second: 00,
        };
        assert_eq!(business_day_shift(t, BusinessDayConvention::NULL), t);
    }
}
