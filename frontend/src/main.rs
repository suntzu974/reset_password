use yew::prelude::*;
use web_sys::UrlSearchParams;

#[function_component(ResetPassword)]
fn reset_password() -> Html {
    let token = use_state(|| None::<String>);

    use_effect_with((), {
        let token = token.clone();
        move |_| {
            let location = web_sys::window().unwrap().location();
            let search = location.search().unwrap_or_default(); // ?token=abc123
            let params = UrlSearchParams::new_with_str(&search).unwrap();
            let token_value = params.get("token");
            token.set(token_value);
            || ()
        }
    });

    html! {
        <>
            <h1>{ "Réinitialisation du mot de passe du côte serveur" }</h1>
            {
                if let Some(t) = &*token {
                    html! { <p>{ format!("Token reçu : {}", t) }</p> }
                } else {
                    html! { <p>{ "Aucun token trouvé dans l'URL." }</p> }
                }
            }
        </>
    }
}

fn main() {
    yew::Renderer::<ResetPassword>::new().render();
}
