use clap::{Clap, crate_authors, crate_name, crate_version};

#[derive(Clap)]
#[clap(
    author = crate_authors!(),
    name = crate_name!(),
    version = crate_version!(),
    about = "Sample app with some health and ready endpoints for k8 probes.",
)]
pub struct Configuration {

    #[clap(
        long, 
        about = "Seconds the app waites till the ready endpoint succeed.",
        env = "PROBES_APP_SECONDS_TILL_READY"
    )]
    pub seconds_till_ready : u64,

    #[clap(
        long, 
        about = "Seconds the app waites till the liveness endpoint starts failing.",
        env = "PROBES_APP_SECONDS_TILL_FAULTY"
    )]
    pub seconds_till_faulty : u64,
}