mod app;
mod renderer;
mod camera;
mod grid;

pub fn run() -> anyhow::Result<()> {
    app::run()
}
