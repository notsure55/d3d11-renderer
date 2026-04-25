use anyhow::Result;
use renderer::{renderer::*, window::*};

fn main() -> Result<()> {
    Renderer::run(Window::new()?)?;

    Ok(())
}
