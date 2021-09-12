use actix_web_static_files::NpmBuild;

fn main() -> std::io::Result<()> {
    NpmBuild::new("pitunes-frontend")
        .executable("yarn")
        .install()?
        .run("build")?
        .target("pitunes-frontend/build")
        .to_resource_dir()
        .build()?;

    Ok(())
}