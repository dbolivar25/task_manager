mod app_driver;
mod task;
mod task_manager;

use anyhow::*;

use app_driver::*;

fn main() -> Result<()> {
    let mut app = App::new();
    app.run()
}
