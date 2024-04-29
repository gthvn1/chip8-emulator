pub mod camera2d;
pub mod color;
pub mod keys;
pub mod rectangle;
pub mod vector2;

use std::{
    ffi::{c_float, c_void, CString},
    os::raw::{c_char, c_int},
};

use camera2d::Camera2D;
use color::Color;
use rectangle::Rectangle;

extern "C" {
    fn BeginDrawing() -> c_void;
    fn BeginMode2D(camera: Camera2D) -> c_void;
    fn ClearBackground(c: Color) -> c_void;
    fn CloseWindow() -> c_void;
    fn DrawLine(start_x: i32, start_y: i32, end_x: i32, end_y: i32, c: Color) -> c_void;
    fn DrawText(text: *const c_char, x: c_int, y: c_int, fs: c_int, c: Color) -> c_void;
    fn DrawRectangle(x: c_int, y: c_int, w: c_int, h: c_int, color: Color) -> c_void;
    fn DrawRectangleRec(rec: Rectangle, color: Color) -> c_void;
    fn EndDrawing() -> c_void;
    fn EndMode2D() -> c_void;
    fn GetMouseWheelMove() -> c_float;
    fn InitWindow(w: c_int, h: c_int, title: *const c_char) -> c_void;
    fn IsKeyPressed(k: c_int) -> c_int;
    fn SetTargetFPS(fps: c_int) -> c_void;
    fn WindowShouldClose() -> c_int;
}

pub fn begin_drawing() {
    unsafe { BeginDrawing() };
}

pub fn begin_mode_2d(camera: Camera2D) {
    unsafe { BeginMode2D(camera) };
}

pub fn clear_background(color: Color) {
    unsafe { ClearBackground(color) };
}

pub fn close_window() {
    unsafe { CloseWindow() };
}

pub fn draw_line(start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: Color) {
    unsafe {
        DrawLine(
            start_x as c_int,
            start_y as c_int,
            end_x as c_int,
            end_y as c_int,
            color,
        )
    };
}

pub fn draw_rectangle(x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe { DrawRectangle(x as c_int, y as c_int, w as c_int, h as c_int, color) };
}

pub fn draw_rectangle_rec(rec: Rectangle, color: Color) {
    unsafe { DrawRectangleRec(rec, color) };
}

pub fn draw_text(text: String, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    let c_text = CString::new(text).unwrap();
    unsafe {
        DrawText(
            c_text.as_ptr() as *const c_char,
            pos_x as c_int,
            pos_y as c_int,
            font_size as c_int,
            color,
        )
    };
}

pub fn end_drawing() {
    unsafe { EndDrawing() };
}

pub fn end_mode_2d() {
    unsafe { EndMode2D() };
}

pub fn get_mouse_wheel_move() -> f32 {
    let v = unsafe { GetMouseWheelMove() };
    v as f32
}

pub fn init_window(width: i32, height: i32, title: String) {
    let c_title = CString::new(title).unwrap();
    unsafe {
        InitWindow(
            width as c_int,
            height as c_int,
            c_title.as_ptr() as *const c_char,
        )
    };
}

pub fn is_key_pressed(key: i32) -> bool {
    unsafe { IsKeyPressed(key as c_int) != 0 }
}

pub fn set_target_fps(fps: i32) {
    unsafe { SetTargetFPS(fps as c_int) };
}

pub fn window_should_close() -> bool {
    unsafe { WindowShouldClose() != 0 }
}
