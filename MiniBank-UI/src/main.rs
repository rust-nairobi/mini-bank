#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
use sycamore::prelude::*;

mod rust_nairobi_logo;
pub use rust_nairobi_logo::*;

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            div(class="palette-01 frow  row-center height-100vhmin") {
                div(class="frow col-md-1-2") {
                    div(class="svg-50 "){
                        (svg_logo(cx))
                    }
                }
                div(class="frow col-md-1-2") {
                    div(class="frow direction-column row-center width-100") {
                        h3 { "Hello, Rusty MiniBank here!" }
                        h4 { "Make by Rust-Nairobi Developers for Rusteceans" }
                    }
                }
            }
        }
    });
}
