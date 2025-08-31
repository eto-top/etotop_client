#[derive(Debug, Clone, Copy)]
#[repr(i64)]
pub enum CAuthState {
    EnterUrl = 1,
    Login = 2,
    Main = 3,
}
