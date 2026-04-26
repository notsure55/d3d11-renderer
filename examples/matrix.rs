use anyhow::Result;
use renderer::math::*;

fn main() -> Result<()> {
    let mat4x4 = Matrix4x4 {
        m: [
            [6.0, -2.0, -2.0, 1.0],
            [9.0, 1.0, 13.0, 1.0],
            [0.0, -8.0, -8.0, 1.0],
            [1.0, 2.0, 3.0, 1.0],
        ],
    };

    let inverse = mat4x4.inverse();

    let identity = mat4x4 * inverse;

    dbg!(identity);

    Ok(())
}
