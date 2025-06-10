use yew::prelude::*;
use yew_router::prelude::*;

mod routes;
mod pages {
    pub mod forgot_password;
    pub mod reset_password;
}

use routes::Route;
use pages::forgot_password::ForgotPassword;
use pages::reset_password::ResetPassword;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::ForgotPassword => html! { <ForgotPassword /> },
        Route::ResetPassword => html! { <ResetPassword /> },
        Route::NotFound => html! { <p>{ "Page non trouvée" }</p> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
