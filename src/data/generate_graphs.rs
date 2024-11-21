use leptos::logging;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;

pub fn generate_answered_unanswered_histogram(
    canvas_id: &str,
    unresolved: i32,
    resolved: i32,
) -> Result<(), JsValue> {
    leptos::logging::error!(
        "Generating histogram with unanswered: {}, answered: {}",
        unresolved,
        resolved
    );

    // Retrieve the canvas element by ID
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(canvas_id)
        .ok_or_else(|| JsValue::from_str("Canvas element not found"))?;

    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("Failed to cast element to HtmlCanvasElement"))?;

    let backend = CanvasBackend::with_canvas_object(canvas)
        .ok_or_else(|| JsValue::from_str("Failed to initialize canvas backend"))?;
    let root_area = backend.into_drawing_area();

    // Clear the canvas
    root_area
        .fill(&WHITE)
        .map_err(|e| JsValue::from_str(&format!("Error clearing canvas: {}", e)))?;

    let categories = ["Resolved", "Unresolved"];

    let mut chart = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .caption("Resolved vs Unresolved Questions", ("sans-serif", 30))
        .build_cartesian_2d(categories.into_segmented(), 0..(resolved + unresolved))
        .map_err(|e| JsValue::from_str(&format!("Error building chart: {}", e)))?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .x_labels(2)
        .draw()
        .map_err(|e| JsValue::from_str(&format!("Error drawing chart mesh: {}", e)))?;

    let data = vec![("Resolved", resolved), ("Unresolved", unresolved)];

    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(RED.filled()) // Style the bars
                .data(data.iter().map(|(key, count)| (key, *count))),
        )
        .map_err(|e| JsValue::from_str(&format!("Error drawing histogram: {}", e)))?;

    root_area
        .present()
        .map_err(|e| JsValue::from_str(&format!("Error presenting chart: {}", e)))?;
    Ok(())
}
