use crate::api;
use crate::components::ProductCard;
use crate::types::{CartProduct, Product};
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

struct State {
    products: Vec<Product>,
    get_products_error: Option<Error>,
    get_products_loaded: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub cart_products: Vec<CartProduct>,
    pub on_add_to_cart: Callback<Product>,
}

pub struct Home {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum Msg {
    GetProducts,
    GetProductsSuccess(Vec<Product>),
    GetProductsError(Error),
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let products = vec![];

        link.send_message(Msg::GetProducts);

        Self {
            props,
            state: State {
                products,
                get_products_error: None,
                get_products_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetProducts => {
                self.state.get_products_loaded = false;
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<Vec<Product>>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(products) => Msg::GetProductsSuccess(products),
                                Err(err) => Msg::GetProductsError(err),
                            }
                        });

                self.task = Some(api::get_products(handler));
                true
            }
            Msg::GetProductsSuccess(products) => {
                self.state.products = products;
                self.state.get_products_loaded = true;
                true
            }
            Msg::GetProductsError(error) => {
                self.state.get_products_error = Some(error);
                self.state.get_products_loaded = true;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let products: Vec<Html> = self
            .state
            .products
            .iter()
            .map(|product: &Product| {
                html! {
                  <ProductCard product={product} on_add_to_cart=self.props.on_add_to_cart.clone()/>
                }
            })
            .collect();

        if !self.state.get_products_loaded {
            html! {
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"Loading ..."}</div>
                </div>
            }
        } else if let Some(_) = self.state.get_products_error {
            html! {
              <div>
                <span>{"Error loading products! :("}</span>
              </div>
            }
        } else {
            html! {
                <div class="product_card_list">{products}</div>
            }
        }
    }
}
