mod cli;
mod info;
mod subset;

fn main() {
    #[cfg(debug_assertions)]
    let env = env_logger::Env::default().default_filter_or("info,duanzhang=debug");
    #[cfg(not(debug_assertions))]
    let env = env_logger::Env::default().default_filter_or("warn,duanzhang=info");
    env_logger::init_from_env(env);

    let args = cli::get_args();

    match &args.command {
        cli::Commands::Info { file } => {
            if let Err(e) = info::info(file) {
                log::error!("Error: {}", e);
            }
        }
        cli::Commands::Subset {
            file,
            output,
            charset,
            presets,
        } => {
            if let Err(e) = subset::subset(file, output, charset, presets) {
                log::error!("Error: {}", e);
            }
        }
    }
}
