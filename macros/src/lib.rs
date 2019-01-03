#[macro_export]
macro_rules! hashmap {
    () => {
        std::collections::HashMap::new()
    };
    ( $( $key:expr => $val:expr ),* ) => {
        {
            let mut h = std::collections::HashMap::new();
            $( h.insert($key, $val); )*
            h
        }
    };
    ( $( $key:expr => $val:expr ),*, ) => {
        {
            let mut h = std::collections::HashMap::new();
            $( h.insert($key, $val); )*
            h
        }
    };
}
