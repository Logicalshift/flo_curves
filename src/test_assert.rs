
#[cfg(not(any(test, extra_checks)))]
macro_rules! test_assert {
    ($cond:expr) => ({  });
    ($cond:expr,) => ({  });
    ($cond:expr, $($arg:tt)+) => ({  });
}

#[cfg(any(test, extra_checks))]
macro_rules! test_assert {
    ($cond:expr) => ({ assert!($cond); });
    ($cond:expr,) => ({ assert!($cond); });
    ($cond:expr, $($arg:tt)+) => ({ assert!($cond, $($arg)*); });
}
