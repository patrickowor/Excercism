#[derive(Debug, Eq, PartialEq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let DAY = 60 * 24;
        let val = (((hours * 60 + minutes) % DAY) + DAY) % DAY ;
        Self (val)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new (0, self.0 + minutes )
    }
    pub fn to_string(&self) -> String{
        format!("{:02}:{:02}", self.0 / 60  ,  self.0 % 60  )
    }
    
}

