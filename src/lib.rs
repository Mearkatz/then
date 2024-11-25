#[macro_export]
macro_rules! then {
    ($x: expr, $y: expr) => {{
        if let Some(y) = $x {
            $y
        } else {
            None
        }
    }};
}
