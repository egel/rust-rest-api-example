mod embedded {
    refinery::embed_migrations!("./migrations");
}

pub fn start_migrations() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = postgres::Client::connect(&crate::envs::database_url(), postgres::NoTls)?;
    embedded::migrations::runner().run(&mut client)?;

    Ok(())
}
