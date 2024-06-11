mod cookie;
mod finder;
//mod joke;
mod model;

use cookie::*;
use finder::*;
//use joke::*;
use model::*;

//use std::collections::HashSet;

extern crate serde;
// use gloo_console::log;
use gloo_net::http;
extern crate wasm_bindgen_futures;
use wasm_cookies as cookies;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

//pub type JokeResult = Result<JokeStruct, gloo_net::Error>;
pub type QuestionResult = Result<Question, gloo_net::Error>;

struct App {
    cookie: String,
    //joke: JokeResult,
    question: QuestionResult,
}

/*pub enum Msg {
    GotJoke(QuestionResult),
    GetJoke(Option<String>),
}*/
pub enum Msg {
    GotQuestion(QuestionResult),
    GetQuestion(Option<String>),
}

/*impl App {
    fn refresh_joke(ctx: &Context<Self>, key: Option<String>) {
        let got_joke = JokeStruct::get_joke(key);
        ctx.link().send_future(got_joke);
    }
}*/
impl App {
    fn refresh_question(ctx: &Context<Self>, key: Option<String>) {
        let got_question = Question::get_question(key);
        ctx.link().send_future(got_question);
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let cookie = acquire_cookie();
        //App::refresh_joke(ctx, None);
        App::refresh_question(ctx, None);
        //let joke = Err(gloo_net::Error::GlooError("Loading Joke…".to_string()));
        let question = Err(gloo_net::Error::GlooError("Loading Question...".to_string()));
        //Self { cookie, joke }
        Self { cookie, question }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            /*Msg::GotJoke(joke) => {
                self.joke = joke;
                true
            }*/
            Msg::GotQuestion(question) => {
                self.question = question;
                true
            }
            /*Msg::GetJoke(key) => {
                // log!(format!("GetJoke: {:?}", key));
                App::refresh_joke(ctx, key);
                false
            }*/
            Msg::GetQuestion(key) => {
                App::refresh_question(ctx, key);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cookie = &self.cookie;
        //let joke = &self.joke;
        let question = &self.question;
        html! {
        <>
            <h1>{ "Question-Question" }</h1>
            if false {
                {render_cookie(cookie)}
            }
            /*if let Ok(ref joke) = joke {
                <Joke joke={joke.clone()}/>
            }*/
            if let Ok(ref question) = question {
                <QuestionComp question={question.clone()}/>
            }
            /*if let Err(ref error) = joke {
                <div>
                    <span class="error">{format!("Server Error: {error}")}</span>
                </div>
            }*/
            if let Err(ref error) = question {
                <div>
                    <span class="error">{format!("Server Error: {error}")}</span>
                </div>
            }
            <div>
                <button onclick={ctx.link().callback(|_| Msg::GetQuestion(None))}>{"Tell me another!"}</button>
            </div>
            <Finder on_find={ctx.link().callback(Msg::GetQuestion)}/>
        </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
