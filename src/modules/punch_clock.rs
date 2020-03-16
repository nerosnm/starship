use punch_clock::{sheet::SheetStatus, Sheet};

use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::punch_clock::PunchClockConfig;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const PC_PREFIX: &str = "since ";

    let mut module = context.new_module("punch_clock");
    let config: PunchClockConfig = PunchClockConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    module.set_style(config.style);
    module.get_prefix().set_value(PC_PREFIX);

    if let SheetStatus::PunchedIn(start) = Sheet::load_default().ok()?.status() {
        module.create_segment("symbol", &config.symbol);
        module.create_segment(
            "status",
            &SegmentConfig::new(&*start.format("%H:%M").to_string()),
        );

        Some(module)
    } else {
        None
    }
}
