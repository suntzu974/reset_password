use yew::prelude::*;
use web_sys::UrlSearchParams;

#[function_component(ResetPassword)]
pub fn reset_password() -> Html {
    let token = use_state(|| None::<String>);

    use_effect_with((), {
        let token = token.clone();
        move |_| {
            let location = web_sys::window().unwrap().location();
            let search = location.search().unwrap_or_default(); // ?token=abc
            let params = UrlSearchParams::new_with_str(&search).unwrap();
            token.set(params.get("token"));
            || ()
        }
    });

    html! {
        <div>
            <h2>{"Réinitialiser le mot de passe"}</h2>
            {
                if let Some(token) = &*token {
                    html! {
                        <>
                            <p>{format!("Token: {}", token)}</p>
                            <form>
                                <label>{"Nouveau mot de passe :"}</label>
                                <input type="password" />
                                <button type="submit">{"Réinitialiser"}</button>
                            </form>
                        </>
                    }
                } else {
                    html! { <p>{"Token manquant ou invalide."}</p> }
                }
            }
        </div>
    }
}
