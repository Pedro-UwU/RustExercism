use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = Clock::format_time(hours, minutes);
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let min_to_hours = minutes / 60;
        let rem = minutes % 60;
        let mut hours = self.hours + min_to_hours;
        let mut minutes = self.minutes + rem;
        (hours, minutes) = Clock::format_time(hours, minutes);
        println!("min_to_hours: {}, rem: {}, hours: {}, minutes: {}", min_to_hours, rem, hours, minutes);
        
        Clock { hours, minutes }
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    fn format_time(mut hours: i32, mut minutes: i32) -> (i32, i32) {
        let mut extra_hours = 0;
        while minutes < 0 { 
            extra_hours -= 1;
            minutes += 60; 
        }
    
        while minutes >= 60 { 
            extra_hours += 1;
            minutes -= 60;
        }
        hours += extra_hours;
        
        while hours < 0 { hours += 24; }
        while hours >= 24 { hours -= 24; }


        (hours, minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), Error> {
        println!("{}", self);
        Ok(())
    }

}
