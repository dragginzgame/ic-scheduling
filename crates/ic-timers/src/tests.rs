// Keep event names and exact values visible while retaining overflow refusal.
macro_rules! assert_counters {
    ($observed:expr, $(($field:ident, $expected:expr)),+ $(,)?) => {{
        let counters = $observed;
        $(assert_eq!(
            counters.$field().exact_total(),
            Ok($expected),
            "{} counter", stringify!($field),
        );)+
    }};
}

pub(crate) use assert_counters;
