use yew::prelude::*;

mod home;
mod product_detail;

pub use home::Home;
pub use product_detail::ProductDetail;

fn spinner() -> Html {
    html! {
        <div class="loading_spinner_container">
            <div class="loading_spinner"></div>
            <div class="loading_spinner_text">{"Loading ..."}</div>
        </div>
    }
}

fn error(error: &str) -> Html {
    html! {
        <div>
            <span>{error}</span>
        </div>
    }
}
