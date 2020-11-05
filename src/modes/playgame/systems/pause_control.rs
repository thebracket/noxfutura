use super::super::GameStateResource;
use super::super::RunState;
use crate::modes::playgame::DesignMode;
use bengine::VirtualKeyCode;
use legion::*;
use nox_planet::MiningMode;

#[system]
pub fn pause_control(
    #[resource] state: &mut GameStateResource,
    #[resource] run_state: &mut RunState,
) {
    if let Some(key) = state.keycode {
        match run_state {
            RunState::Design { mode } => match mode {
                DesignMode::Mining { .. } => match key {
                    VirtualKeyCode::H => {
                        *run_state = RunState::Design {
                            mode: DesignMode::Mining {
                                mode: MiningMode::Channel,
                            },
                        };
                    }
                    VirtualKeyCode::R => {
                        *run_state = RunState::Design {
                            mode: DesignMode::Mining {
                                mode: MiningMode::Ramp,
                            },
                        };
                    }
                    VirtualKeyCode::U => {
                        *run_state = RunState::Design {
                            mode: DesignMode::Mining {
                                mode: MiningMode::Up,
                            },
                        };
                    }
                    VirtualKeyCode::J => {
                        *run_state = RunState::Design {
                            mode: DesignMode::Mining {
                                mode: MiningMode::Down,
                            },
                        };
                    }
                    VirtualKeyCode::K => {
                        *run_state = RunState::Design {
                            mode: DesignMode::Mining {
                                mode: MiningMode::UpDown,
                            },
                        };
                    }
                    VirtualKeyCode::X => {
                        *run_state = RunState::Design {
                            mode: DesignMode::Mining {
                                mode: MiningMode::Clear,
                            },
                        };
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        match key {
            VirtualKeyCode::Grave => *run_state = RunState::Paused,
            VirtualKeyCode::Key1 => *run_state = RunState::SlowMo,
            VirtualKeyCode::Key2 => *run_state = RunState::Running,
            VirtualKeyCode::Key3 => *run_state = RunState::FullSpeed,
            VirtualKeyCode::T => {
                *run_state = RunState::Design {
                    mode: DesignMode::Lumberjack,
                }
            }
            VirtualKeyCode::B => {
                *run_state = RunState::Design {
                    mode: DesignMode::Buildings { bidx: 0, vox: None },
                };
            }
            VirtualKeyCode::D => {
                *run_state = RunState::Design {
                    mode: DesignMode::Mining {
                        mode: MiningMode::Dig,
                    },
                };
            }
            VirtualKeyCode::S => {
                *run_state = RunState::Design {
                    mode: DesignMode::SettlerList,
                };
            }
            _ => {}
        }
    }
}
