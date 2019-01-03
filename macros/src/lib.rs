#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),*, ) => {
        hashmap!($($key => $val),*)
    };
    ( $( $key:expr => $val:expr ),* ) => {
        {
            let mut h = std::collections::HashMap::new();
            $( h.insert($key, $val); )*
            h
        }
    };
}
