use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ScheduleTime {
    Work,
    Sleep,
    Leisure,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct WorkSchedule {
    pub hours: [ScheduleTime; 24],
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Shift {
    Morning,
    Day,
    Night,
}

impl WorkSchedule {
    pub fn new(shift: Shift) -> Self {
        match shift {
            Shift::Morning => Self {
                hours: [
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                ],
            },
            Shift::Night => Self {
                hours: [
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                ],
            },
            Shift::Day => Self {
                hours: [
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Sleep,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Work,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                    ScheduleTime::Leisure,
                ],
            },
        }
    }
}
