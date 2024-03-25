use dioxus::prelude::*;

use crate::dashboard;

// Dashboard component that renders a grid of divs
pub fn Dashboard() -> Element {
    let outer_color = use_signal(|| "#2196F3".to_string());
    let inner_color = use_signal(|| "#9621F3".to_string());

    rsx! {
        div { style: "grid-template-columns: auto auto auto;
            grid-template-rows: auto auto;
            grid-auto-flow: dense;
            display: grid;
            gap: 20px;
            padding: 15px;
            height: 500px;
            background-color: {outer_color};",

            div { style: "grid-row: span 1; grid-column: span 2; background-color: {inner_color};", "Hello there! 1" }
            dashboard::tileA { title: "Hello there! 2" } 
            div { style: "grid-row: span 2; grid-column: span 1; background-color: {inner_color};", "Hello there! 3" }
            div { style: "grid-row: span 1; grid-column: span 1; background-color: {inner_color};", "Hello there! 4" }
            div { style: "grid-row: span 1; grid-column: span 1; background-color: {inner_color};", "Hello there! 5" }
            div { style: "grid-row: span 2; grid-column: span 2; background-color: {inner_color};", "Hello there! 6" }            
            dashboard::tileA { title: "Hello there! 7" }
        }
    }
}


// TileA Component that renders a div with a title
// To pass a value to a component, you need to use the `component` attribute
#[component]
pub fn tileA(title: String) -> Element {
    // A rounded div with a blue background and a boarder of 1px
    let gridspan = use_signal(|| "grid-row: span 2; grid-column: span 2;".to_string());

    rsx! {
        div { style: "{gridspan} border: 1px solid black; border-radius: 15px; background-color: blue; padding: 10px", "{title}" }
    }
}