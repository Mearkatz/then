#[macro_export]
macro_rules! then {
    ($x: expr, $y: expr) => {{
        if $x {
            Some($y)
        } else {
            None
        }
    }};
}
