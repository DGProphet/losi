use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Widget {
    pub name: String,
    pub widget_type: WidgetType,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum WidgetType {
    Clock,
    Weather,
    SystemInfo,
    Custom(String),
}

impl Widget {
    #[allow(dead_code)]
    pub fn new(name: &str, widget_type: WidgetType, x: i32, y: i32) -> Self {
        Self {
            name: name.to_string(),
            widget_type,
            x,
            y,
            width: 200,
            height: 100,
            visible: true,
        }
    }
}
