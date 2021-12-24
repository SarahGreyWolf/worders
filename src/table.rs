use super::util::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlElement, HtmlTableCellElement, HtmlTableElement};

pub struct Table {
    size: [u32; 2],
    element: HtmlTableElement,
    cells: Vec<Cell>,
}

impl Table {
    pub fn new(
        width: u32,
        height: u32,
        _cell_size: u32,
        element: Option<HtmlElement>,
        class: Option<&str>,
        id: Option<&str>,
    ) -> Self {
        let table = document().create_element("table").unwrap();
        if let Some(class) = class {
            table.set_class_name(class);
        }
        if let Some(id) = id {
            table.set_id(id);
        }
        let window = window();
        let w_height = window.inner_height().unwrap().as_f64().unwrap() as u32;
        let w_width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let dom_dimension = if w_height > w_width {
            w_width
        } else {
            w_height
        };
        let mut cells: Vec<Cell> = vec![];
        for row in 0..height {
            let new_row = document()
                .create_element("tr")
                .expect(&format!("Failed to create row at {:#}", row));
            for column in 0..width {
                let cell = document()
                    .create_element("td")
                    .expect(&format!("Failed to create cell at {:#},{:#}", column, row));
                cell.set_id(&format!("{:#}", column + row * width));
                let cell_element = cell.dyn_ref::<HtmlTableCellElement>().expect(&format!(
                    "Element at {:#},{:#} was not a HtmlElement",
                    column, row
                ));
                cell_element.set_width(&format!("{:#}", dom_dimension / width - 4));
                cell_element.set_height(&format!("{:#}", dom_dimension / height - 4));
                cells.push(Cell::new(column, row, cell_element.clone()));
                new_row.append_child(&cell_element).unwrap();
            }
            table.append_child(&new_row).unwrap();
        }
        let table_element = table
            .dyn_ref::<HtmlTableElement>()
            .expect("Element was not a table");
        if let Some(element) = element {
            element
                .append_child(&table)
                .expect("Failed to append table to element");
        } else {
            body()
                .append_child(&table)
                .expect("Failed to append table to body");
        }
        Table {
            size: [width, height],
            element: table_element.clone(),
            cells,
        }
    }

    pub fn new_no_height_aspect(
        width: u32,
        height: u32,
        cell_size: u32,
        element: Option<HtmlElement>,
        class: Option<&str>,
        id: Option<&str>,
    ) -> Self {
        let table = document().create_element("table").unwrap();
        if let Some(class) = class {
            table.set_class_name(class);
        }
        if let Some(id) = id {
            table.set_id(id);
        }
        let window = window();
        let w_height = window.inner_height().unwrap().as_f64().unwrap() as u32;
        let w_width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let dom_dimension = if w_height > w_width {
            w_width
        } else {
            w_height
        };
        let mut cells: Vec<Cell> = vec![];
        for row in 0..height {
            let new_row = document()
                .create_element("tr")
                .expect(&format!("Failed to create row at {:#}", row));
            for column in 0..width {
                let cell = document()
                    .create_element("td")
                    .expect(&format!("Failed to create cell at {:#},{:#}", column, row));
                cell.set_id(&format!("{:#}", column + row * width));
                let cell_element = cell.dyn_ref::<HtmlTableCellElement>().expect(&format!(
                    "Element at {:#},{:#} was not a HtmlElement",
                    column, row
                ));
                cell_element.set_width(&format!("{:#}", (dom_dimension / width) / 2));
                cell_element.set_height(&format!("{:#}", (dom_dimension / width) / 2));
                cells.push(Cell::new(column, row, cell_element.clone()));
                new_row.append_child(&cell_element).unwrap();
            }
            table.append_child(&new_row).unwrap();
        }
        let table_element = table
            .dyn_ref::<HtmlTableElement>()
            .expect("Element was not a table");
        if let Some(element) = element {
            element
                .append_child(&table)
                .expect("Failed to append table to element");
        } else {
            body()
                .append_child(&table)
                .expect("Failed to append table to body");
        }
        Table {
            size: [width, height],
            element: table_element.clone(),
            cells,
        }
    }

    pub fn get_cell(&mut self, x: u32, y: u32) -> &mut Cell {
        &mut self.cells[(x + y * self.size[1]) as usize]
    }

    pub fn get_cells(&mut self) -> Vec<Cell> {
        self.cells.clone()
    }

    pub fn set_cells(&mut self, cells: Vec<Cell>) {
        self.cells = cells;
    }
}

#[derive(Clone)]
pub struct Cell {
    pub position: [u32; 2],
    pub element: HtmlTableCellElement,
    background_image: String,
    background_colour: String,
}

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
            background_colour: "".to_string(),
        }
    }

    fn set_image(&mut self, uri: &str) {
        self.background_image = uri.to_string();
    }

    fn set_colour(&mut self, colour: &str) {
        self.background_colour = colour.to_string();
    }

    pub fn set_callback(&mut self, cb: Box<dyn FnMut()>) {
        let closure = Closure::wrap(cb as Box<dyn FnMut()>);
        self.element
            .set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }
}
