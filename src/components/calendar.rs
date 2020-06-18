use super::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "a6464855-c959-4eb3-be05-dec8301b06b8"]
pub struct Calendar {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Calendar {
    pub fn get_date_time(&self) -> String {
        format!(
            "{:02} {} {} {:02}:{:02}",
            self.day+1,
            match self.month {
                0 => "Jan",
                1 => "Feb",
                2 => "Mar",
                3 => "Apr",
                4 => "May",
                5 => "Jun",
                6 => "Jul",
                7 => "Aug",
                8 => "Sep",
                9 => "Oct",
                10 => "Nov",
                _ => "Dec"
            },
            self.year,
            self.hour,
            self.minute
        )
    }

    pub fn calculate_sun_moon(&self) -> (f32, f32, f32) {
        let minutes_fraction = self.minute as f32 / 60.0;
        let hours_fraction = self.hour as f32 + minutes_fraction - 6.0;
        let time_overall = hours_fraction / 24.0;
        let time_as_radians = time_overall * 6.28319;
        let x = f32::cos(time_as_radians);
        let y = f32::sin(time_as_radians);
        (x, y, 0.0)
    }
}