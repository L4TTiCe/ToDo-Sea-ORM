use crate::errors::Error;

pub fn get_env_var(env_var_name: String) -> Result<String, Error> {
    std::env::var(&env_var_name)
        .map_err(|_| {
            error!("ENV VARIABLE for `{}` is not set", env_var_name);
            Error::EnvironmentVariableNotSet(env_var_name)
        })
}
