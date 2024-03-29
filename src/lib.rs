#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn scream(input: String) -> String {
        let mut s = input.to_uppercase();
        s.push_str("!!1!");
        s.into()
    }
}

bindings::export!(Component with_types_in bindings);
