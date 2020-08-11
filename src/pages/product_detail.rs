use crate::api;
use crate::components::AtcButton;
use crate::types::Product;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

struct State {
    product: Option<Product>,
    get_product_error: Option<Error>,
    get_product_loaded: bool,
}

pub struct ProductDetail {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: i32,
    pub on_add_to_cart: Callback<Product>,
}

pub enum Msg {
    GetProduct,
    GetProductSuccess(Product),
    GetProductError(Error),
}

impl Component for ProductDetail {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetProduct);

        Self {
            props,
            state: State {
                product: None,
                get_product_error: None,
                get_product_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetProduct => {
                let handler = self
                    .link
                    .callback(move |response: api::FetchResponse<Product>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(product) => Msg::GetProductSuccess(product),
                            Err(err) => Msg::GetProductError(err),
                        }
                    });

                self.task = Some(api::get_product(self.props.id, handler));
                true
            }
            Msg::GetProductSuccess(product) => {
                self.state.product = Some(product);
                self.state.get_product_loaded = true;
                true
            }
            Msg::GetProductError(error) => {
                self.state.get_product_error = Some(error);
                self.state.get_product_loaded = true;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let Some(ref product) = self.state.product {
            html! {
                <div class="product_detail_container">
                    <img class="product_detail_image" src={&product.image}/>
                    <div class="product_card_name">{&product.name}</div>
                    <div style="margin: 10px 0; line-height: 24px;">{&product.description}</div>
                    <div class="product_card_price">{"$"}{&product.price}</div>
                    <AtcButton product=product.clone() on_add_to_cart=self.props.on_add_to_cart.clone() />
                </div>
            }
        } else if !self.state.get_product_loaded {
            html! {
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"Loading ..."}</div>
                </div>
            }
        } else {
            html! {
                <div>
                    <span>{"Error loading product! :("}</span>
                </div>
            }
        }
    }
}
