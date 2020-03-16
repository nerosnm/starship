use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PunchClockConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub status: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PunchClockConfig<'a> {
    fn new() -> Self {
        PunchClockConfig {
            symbol: SegmentConfig::new("🕰  "),
            status: SegmentConfig::default(),
            style: Color::Purple.bold(),
            disabled: false,
        }
    }
}
