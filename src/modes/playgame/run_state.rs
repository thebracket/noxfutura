use nox_components::MiningMode;

#[derive(PartialEq, Clone)]
pub enum RunState {
    Paused,
    SlowMo,
    Running,
    FullSpeed,
    Design { mode: DesignMode },
}

#[derive(PartialEq, Clone)]
pub enum DesignMode {
    Lumberjack,
    Buildings { bidx: i32, vox: Option<usize> },
    Mining { mode: MiningMode },
    SettlerList,
    BuildingInfo { id: usize },
    Construction,
}
