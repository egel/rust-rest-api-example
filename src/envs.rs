use std::env;

/// Name of the environment variable holding the database connection URL.
const DATABASE_URL_KEY: &str = "SERVER_DATABASE_URL";
/// Name of the environment variable holding the application environment name.
const ENVIRONMENT_KEY: &str = "SERVER_ENVIRONMENT";
const HOST_KEY: &str = "SERVER_HOST";
const PORT_KEY: &str = "SERVER_PORT";

/// Possible application environments.
pub const ENVIRONMENT_OPTIONS: [&str; 3] = ["develop", "production", "testing"];

/// Environments in which the `.env` file is loaded.
///
/// Must be a subset of [`ENVIRONMENT_OPTIONS`]. All variables for the other
/// environments must be provided as real environment variables.
const DOTENV_ENVIRONMENTS: [&str; 1] = ["develop"];

/// Validates the `ENVIRONMENT` value (fail fast on typos) and returns it.
fn current_environment() -> Option<String> {
    let environment = env::var(ENVIRONMENT_KEY).ok()?;

    if !ENVIRONMENT_OPTIONS.contains(&environment.as_str()) {
        panic!(
            "unknown ENVIRONMENT value: `{environment}`, allowed values are: {}",
            ENVIRONMENT_OPTIONS.join(", ")
        );
    }

    Some(environment)
}

fn missing(varname: &str) -> String {
    std::fmt::format(format_args!("{} environment variable is not set", varname))
}

/// Loads the `.env` file, but only in the environments from
/// [`DOTENV_ENVIRONMENTS`] (currently only `develop`).
///
/// For any other environment (or when the file does not exist) nothing is
/// loaded and all variables must be provided as real environment variables.
pub fn load_dotenv() {
    if current_environment()
        .is_some_and(|environment| DOTENV_ENVIRONMENTS.contains(&environment.as_str()))
    {
        dotenvy::dotenv().expect(".env file could not be loaded in the develop environment");
    }
}

/// Database connection URL, e.g. `postgres://user:pass@localhost:5432/mydb`.
///
/// Panics at startup when the variable is not set.
pub fn database_url() -> String {
    env::var(DATABASE_URL_KEY).unwrap_or_else(|_| panic!("{}", missing(DATABASE_URL_KEY)))
}

/// Application environment name.
///
/// Returns one of [`ENVIRONMENT_OPTIONS`]; when the variable is not set,
/// defaults to `develop`.
pub fn environment() -> String {
    current_environment().unwrap_or_else(|| panic!("{}", missing(ENVIRONMENT_KEY)))
}

/// Server hostname
pub fn host() -> String {
    env::var(HOST_KEY).unwrap_or_else(|_| panic!("{}", missing(HOST_KEY)))
}

/// Server port
pub fn port() -> String {
    env::var(PORT_KEY).unwrap_or_else(|_| panic!("{}", missing(PORT_KEY)))
}
