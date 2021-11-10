use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{HtmlTableElement, HtmlTableCellElement, HtmlElement};
use super::util::*;

fn window() -> web_sys::Window {
    web_sys::window().expect("No global window exists")
}

fn document() -> web_sys::Document {
    window().document().expect("No Document for window exists")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("No body for document exists")
}

#[wasm_bindgen]
pub struct Table {
    size: [u32; 2],
    element: HtmlTableElement,
    cells: Vec<Cell>
}

#[wasm_bindgen]
impl Table {
    pub fn new(width: u32, height: u32, cell_size: u32, element: Option<HtmlElement>) -> Self {
        let table = if let Some(element) = element {
            let table = document().create_element("table").unwrap();
            element.append_child(&table).expect("Failed to append table to element");
            table
        } else {
            document().create_element("table").unwrap()
        };
        let mut cells: Vec<Cell> = vec![];
        table.set_id("game");
        for y in 0..height {
            let row = document().create_element("tr").expect(&format!("Failed to create row at {:#}", y));
            for x in 0..width {
                let cell = document().create_element("td").expect(&format!("Failed to create cell at {:#},{:#}", x, y));
                cell.set_id(&format!("{:#}", x + y * width));
                cell.set_class_name("inactive");
                let cell_element = cell.dyn_ref::<HtmlTableCellElement>().expect("Element was not a HtmlElement");
                cells.push(Cell::new(x, y, cell_element.clone()));
                row.append_child(&cell).unwrap();
            }
            table.append_child(&row).unwrap();
        }
        let table_element = table.dyn_ref::<HtmlTableElement>().expect("Element was not a table");
        body().append_child(&table).expect("Failed to append table to body");
        Table {
            size: [width, height],
            element: table_element.clone(),
            cells
        }
    }
}

#[wasm_bindgen]
pub struct Cell {
    position: [u32; 2],
    element: HtmlTableCellElement,
    background_image: String,
    background_colour: String
}

#[wasm_bindgen]
impl Cell {
    pub fn new(x: u32, y: u32, element: HtmlTableCellElement) -> Self {
        let cell_clone = element.clone();
        let onclick = Closure::wrap(Box::new(move || {
            if cell_clone.class_name() == "inactive" {
                cell_clone.set_class_name("active");
            } else {
                cell_clone.set_class_name("inactive");
            }
        }) as Box<dyn FnMut()>);
        element.set_onclick(Some(onclick.as_ref().unchecked_ref()));
        onclick.forget();
        Cell {
            position: [x, y],
            element,
            background_image: "".to_string(),
            background_colour: "".to_string()
        }
    }

    fn set_image(&mut self, uri: &str) {
        self.background_image = uri.to_string();
    }

    fn set_colour(&mut self, colour: &str) {
        self.background_colour = colour.to_string();
    }
}
