use wasm_bindgen::prelude::*;

pub enum Status {
    Primary,
    Success,
    Warning,
    Danger,
}

impl Into<String> for Status {
    fn into(self) -> String {
        match self {
            Status::Primary => "primary".to_owned(),
            Status::Success => "success".to_owned(),
            Status::Warning => "warning".to_owned(),
            Status::Danger => "danger".to_owned(),
        }
    }
}

pub enum Position {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Into<String> for Position {
    fn into(self) -> String {
        match self {
            Position::TopLeft => "top-left".to_owned(),
            Position::TopCenter => "top-center".to_owned(),
            Position::TopRight => "top-right".to_owned(),
            Position::BottomLeft => "bottom-left".to_owned(),
            Position::BottomCenter => "bottom-center".to_owned(),
            Position::BottomRight => "bottom-right".to_owned(),
        }
    }
}

#[wasm_bindgen(
    inline_js = "export function uikit_notification(msg,pos,stat,tout) { UIkit.notification(msg,{pos:pos,status:stat,timeout:tout}); }"
)]
extern "C" {
    fn uikit_notification(message: String, position: String, status: String, timeout: u32);
}

pub fn notification(message: String, position: Position, status: Status, timeout: Option<u32>) {
    uikit_notification(
        message,
        position.into(),
        status.into(),
        timeout.unwrap_or(5000),
    );
}
