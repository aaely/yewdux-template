mod state;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use state::{Entry, Filter, State};

#[function_component]
fn Header() -> Html {
    let onkeypress = Dispatch::<State>::new().reduce_mut_callback_with(|s, e: KeyboardEvent| {
        if e.key() == "Enter" {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            input.set_value("");
            if !value.is_empty() {
                let entry = Entry {
                    description: value.trim().to_string(),
                    completed: false,
                    editing: false,
                }
                s.entries.push(entry);
            }
        }
    });
}