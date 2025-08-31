#[macro_export]
macro_rules! lock_field {
    (read $arc:expr, $field:ident) => {
        $arc.read().expect("lock poisoned").$field.clone()
    };
    (write $arc:expr, $field:ident = $val:expr) => {
        $arc.write().expect("lock poisoned").$field = $val
    };
}
