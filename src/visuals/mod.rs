mod app;
mod renderer;
mod camera;
mod grid;
mod units;

pub fn run() -> anyhow::Result<()> {
    app::run()
}
