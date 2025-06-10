use yew::prelude::*;
use gloo_net::http::Request;
use gloo::console::log;

#[function_component(ForgotPassword)]
pub fn forgot_password() -> Html {
    let email = use_state(|| "".to_string());

    let onsubmit = {
        let email = email.clone();
        Callback::from(move |e: web_sys::SubmitEvent| {
            e.prevent_default(); // ✅ important : empêche le rechargement de la page
            let email = (*email).clone();

            wasm_bindgen_futures::spawn_local(async move {
                let request = Request::post("/api/forgot-password")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(format!("email={}", email));

                match request {
                    Ok(req) => {
                        match req.send().await {
                            Ok(resp) => {
                                if resp.ok() {
                                    log!("Lien envoyé !");
                                } else {
                                    log!("Erreur serveur !");
                                }
                            }
                            Err(err) => {
                                log!(format!("Erreur réseau : {:?}", err));
                            }
                        }
                    }
                    Err(err) => {
                        log!(format!("Erreur lors de la création de la requête : {:?}", err));
                    }
                }
            });
        })
    };


    html! {
        <form onsubmit={onsubmit}>
            <label for="email">{"Email :"}</label>
            <input id="email" type="email" value={(*email).clone()} />
            <button type="submit">{"Envoyer"}</button>
        </form>
    }
}
