use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/forgot-password")]
    ForgotPassword,
    #[at("/reset-password")]
    ResetPassword,
    #[not_found]
    #[at("/404")]
    NotFound,
}
