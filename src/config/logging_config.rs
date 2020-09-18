use amethyst::LoggerConfig;
use std::path::PathBuf;

pub fn logging_development_config() -> LoggerConfig {
    LoggerConfig{
        stdout: amethyst::StdoutLog::Colored,
        level_filter: amethyst::LogLevelFilter::Debug,
        log_file: log_path(),
        allow_env_override: false,
        log_gfx_backend_level: None,
        log_gfx_rendy_level: None,
        module_levels: vec![]
    }
}

fn log_path() -> Option<PathBuf> {
    let mut path = PathBuf::new();
    path.push("logfiles/game_log.log");
    Some(path)
}