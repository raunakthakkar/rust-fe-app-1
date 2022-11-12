use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Action {
    Add,
    Remove,
    ChangeInput(String),
}
struct ClassComponent {
    counter: i64,
    input_value: String,
}

impl Component for ClassComponent {
    type Message = Action;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            counter: 0,
            input_value: "".to_string(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let on_change = Callback::from(|e: Event| {
            let input = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            Action::ChangeInput(input);
            return;
        });
        html! {
            <div>
                <button onclick={link.callback(|_| Action::Add)}>{ "+1" }</button>
                <p>{ self.counter }</p>
                <button onclick={link.callback(|_| Action::Remove)}>{ "+1" }</button>
                <br/>
                <br/>
                <input  onchange={on_change} placeholder={"enter a number "} />
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Action::Add => {
                let is_num = &self.input_value.parse::<i64>();
                match is_num {
                    Ok(val) => self.counter += val,
                    Err(err) => {
                        log!("inside else");
                        self.counter += 1
                    }
                }

                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Action::Remove => {
                let is_num = &self.input_value.parse::<i64>();
                match is_num {
                    Ok(val) => self.counter -= val,
                    Err(_) => self.counter -= 1,
                }
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Action::ChangeInput(value) => {
                log!("at change event", &value);
                self.input_value = value.clone();
                log!("self", &self.input_value);
                return true;
            }
        }
    }
}

#[function_component(FunctionalComponent)]
fn functional_component() -> Html {
    html! {
        <div>
        {"we'll be fetching data here"}
        </div>
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/other")]
    FunctionalComponent,
    #[at("/")]
    ClassComponent,
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::ClassComponent => html! {<ClassComponent />},
        Route::FunctionalComponent => html! {<FunctionalComponent/>},
    }
}
#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
fn main() {
    println!("Hello, world!");
    yew::start_app::<Main>();
}
