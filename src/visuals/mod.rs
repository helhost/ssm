mod app;
mod renderer;
mod camera;
mod camera_controller;
mod grid;
mod units;

pub fn run() -> anyhow::Result<()> {
    app::run()
}
