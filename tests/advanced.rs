use int_enum::*;

/// Docs for the enum.
#[int_enum(i64)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Advanced {
    /// Docs for negative one.
    NegativeOne = -1,
    /// Docs for twenty.
    Twenty = 20,
    /// Docs for negative five.
    NegativeFive = -5,
    /// Docs for other.
    Other,
}

#[test]
fn advanced_as_int() {
    assert_eq!(Advanced::NegativeOne.as_int(), Some(-1));
    assert_eq!(Advanced::Twenty.as_int(), Some(20));
    assert_eq!(Advanced::NegativeFive.as_int(), Some(-5));

    assert_eq!(Advanced::Other.as_int(), None);
}

#[test]
fn advanced_from_int() {
    assert_eq!(Advanced::from_int(-1), Some(Advanced::NegativeOne));
    assert_eq!(Advanced::from_int(20), Some(Advanced::Twenty));
    assert_eq!(Advanced::from_int(-5), Some(Advanced::NegativeFive));

    assert_eq!(Advanced::from_int(0), None);
}
