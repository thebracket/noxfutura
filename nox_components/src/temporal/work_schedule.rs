use crate::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "dc6e7909-5bce-4126-b1ec-672930c5af54"]
pub enum ScheduleTime {
    Work,
    Sleep,
    Leisure,
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "d3caa080-defb-4a8f-b80d-ef10fc4e85d4"]
pub struct WorkSchedule {
    pub hours: [ScheduleTime; 24],
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "c51516f4-7816-4fd7-89ab-7fcbb43cf355"]
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
