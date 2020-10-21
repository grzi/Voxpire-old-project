use crate::utilities::traits::Tickable;

pub struct TimeResource {
    pub day_tick: u128,
    pub day: u8,
    pub week: u8,
    pub year: u16
}

impl TimeResource {
    pub fn new() -> Self {
        TimeResource {
            day_tick: 0,
            day: 1,
            week: 1,
            year: 0
        }
    }

    pub fn new_with_values(day_tick: u128, day:u8, week: u8, year: u16) -> Self {
        TimeResource {
            day_tick,
            day,
            week,
            year
        }
    }

    fn update_time(&mut self){
        if self.day_tick >= 24 {
            self.day_tick = 0;
            self.day += 1;
        }

        if self.day > 7 {
            self.day = 1;
            self.week += 1;
        }

        if self.week > {if self.year % 7 == 0 { 52 } else { 53 }} {
            self.week = 1;
            self.year += 1;
        }
    }
}

impl Default for TimeResource {
    fn default() -> Self {
        TimeResource::new()
    }
}

impl Tickable for TimeResource {
    fn tick(&mut self) {
        self.day_tick += 1;
        self.update_time();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24_tick_to_switch_day(){
        let mut time = TimeResource::new();
        assert_eq!(1, time.day);
        for _i in 0..24 {
            assert_eq!(1, time.day);
            time.tick();
        }
        assert_eq!(2, time.day);
    }

    #[test]
    fn test_7_days_is_a_week(){
        let mut time = TimeResource::new();
        assert_eq!(1, time.week);
        for _i in 0..7 {
            for _j in 0..24 {
                assert_eq!(1, time.week);
                time.tick();
            }
        }
        assert_eq!(1, time.day);
        assert_eq!(2, time.week);
    }

    #[test]
    fn test_52_weeks_in_a_year_when_year_is_not_a_7_divisor(){
        let mut time = TimeResource::new();
        assert_eq!(0, time.year);
        for _i in 0..52{
            for _i in 0..7 {
                for _j in 0..24 {
                    assert_eq!(0, time.year);
                    time.tick();
                }
            }
        }
        assert_eq!(1, time.day);
        assert_eq!(1, time.week);
        assert_eq!(1, time.year);
    }

    #[test]
    fn test_53_weeks_in_a_year_when_year_is_a_7_divisor(){
        let mut time = TimeResource::new_with_values(0, 1, 1, 7);
        assert_eq!(7, time.year);
        for _i in 0..52{
            for _i in 0..7 {
                for _j in 0..24 {
                    assert_eq!(7, time.year);
                    time.tick();
                }
            }
        }
        assert_eq!(1, time.day);
        assert_eq!(1, time.week);
        assert_eq!(8, time.year);
    }
}



