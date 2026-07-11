mod laravel;

use anyhow::Result;
use laravel::Laravel;

use crate::core::AppContext;

pub struct DriverCommand {
    pub command: clap::Command,
    pub handler: fn(&clap::ArgMatches, &AppContext) -> Result<()>,
}

pub trait Driver {
    fn name(&self) -> &'static str;
    fn commands(&self) -> Vec<DriverCommand>;
}

pub fn drivers() -> &'static [&'static dyn Driver] {
    &[&Laravel]
}
