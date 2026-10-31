use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // tonic_prost_build::compile_protos("proto/task.proto")?;
    tonic_prost_build::configure()
        .build_server(false)
        //.out_dir("src/google")  // you can change the generated code's location
        .compile_protos(
            &["proto/api/v1/task.proto"],
            &["proto/api/v1"], // specify the root location to search proto dependencies
        )?;
    Ok(())
}