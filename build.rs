use std::boxed::Box;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use glob::glob;
use rayon::prelude::*;

// Example custom build script.
fn main() -> Result<(), Box<dyn Error>> {
    let mut compiler = shaderc::Compiler::new().unwrap();

    for path in glob("./src/**/*.vert")? {
        let path = path?;
        let mut source = String::new();
        let mut file = File::open(&path)?;
        file.read_to_string(&mut source)?;
        let binary = compiler.compile_into_spirv(
            &mut source,
            shaderc::ShaderKind::Vertex,
            path.to_str().unwrap(),
            "main",
            None,
        )?;
    }

    Ok(())
}
