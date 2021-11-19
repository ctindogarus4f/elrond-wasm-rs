#[macro_export]
macro_rules! rust_biguint {
    ($value:expr) => {{
        num_bigint::BigUint::from($value as u64)
    }};
}

#[macro_export]
macro_rules! managed_biguint {
    ($sc_instance:expr, $value:expr) => {{
        BigUint::managed_from($sc_instance.raw_vm_api(), $value as u64)
    }};
}

#[macro_export]
macro_rules! assert_sc_error {
    ($sc_result:expr, $expected_string:expr) => {{
        assert_eq!($sc_result.err().unwrap().as_bytes(), $expected_string)
    }};
}
