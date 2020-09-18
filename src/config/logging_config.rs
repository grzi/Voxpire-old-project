use amethyst::LoggerConfig;
use std::path::{PathBuf, Path};
use std:: env;
use log::LevelFilter;
use crate::utilities::data_access::create_parents_directories_if_not_exist;

static WARN_MODULES: [&'static str; 5] = ["amethyst_rendy", "amethyst_assets", "rendy_chain", "rendy_factory", "rendy_util"];

pub fn create() -> LoggerConfig {
    let should_set_full_debug = env::args().collect::<String>().contains(&String::from("debugall"));
    let mut modules = Vec::new();
    for module in WARN_MODULES.iter() {
        modules.push(module_level_filter_mask(*module, LevelFilter::Warn, should_set_full_debug));
    }
    modules.push(module_level_filter_mask("amethyst", LevelFilter::Warn, should_set_full_debug));

    LoggerConfig{
        stdout: amethyst::StdoutLog::Colored,
        level_filter: amethyst::LogLevelFilter::Debug,
        log_file: log_path(),
        allow_env_override: false,
        log_gfx_backend_level: None,
        log_gfx_rendy_level: None,
        module_levels: modules
    }
}

fn module_level_filter_mask(module_name: &str, default_level: LevelFilter, full_debug: bool) -> (String, LevelFilter) {
    (String::from(module_name), if full_debug { amethyst::LogLevelFilter::Debug } else {default_level})
}

fn log_path() -> Option<PathBuf> {
    let path = Path::new("logfiles/game_log.log");
    if create_parents_directories_if_not_exist(path).is_ok() {
        let mut path_buf = PathBuf::new();
        path_buf.push(path);
        return Some(path_buf);
    }
    None
}