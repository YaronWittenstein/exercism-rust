#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Clock {
            hours: 0,
            minutes: 0,
        };

        clock.add_minutes(hours * 60 + minutes)
    }

    pub fn to_minutes(&self) -> i32 {
        self.hours * 60 + self.minutes
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        let mut mins: i32 = self.to_minutes() + minutes;

        const DAY_IN_MINS: i32 = 24 * 60;

        if mins < 0 {
            mins = mins + DAY_IN_MINS * (mins.abs() / DAY_IN_MINS);

            if mins < 0 {
                mins += DAY_IN_MINS;
            }
        }

        Clock {
            hours: (mins / 60) % 24,
            minutes: mins % 60,
        }
    }
}
