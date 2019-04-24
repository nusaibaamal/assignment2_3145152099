use std::fmt;
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {
    if hours < 24 && minutes < 60 && hours >= 0 && minutes >= 0 {
      return Clock {
        hours: hours, 
        minutes: minutes,
      };
    }
    let m_add = minutes + (hours * 60);
    Clock {
      hours: 0,
      minutes: 0,
    }.add_minutes(m_add)
  }

  pub fn add_minutes(self, minutes: i32) -> Self {
    let mut h_add = minutes / 60;
    let m_add = minutes % 60;

    if m_add < 60 && (self.minutes + m_add) >= 60 && m_add > 0 {
      h_add += 1;
    } else if m_add < 0 && (self.minutes + m_add) < 0 {
      h_add -= 1;
    }
    Clock::new(
      all(self.hours + h_add, 24),
      all(self.minutes + m_add, 60),
    )
  }
}

fn all(value: i32, around: i32) -> i32 {
  let mut all_value = value;
  if all_value < 0 {
    while all_value < 0 {
      all_value += around;
    }
  } else {
    all_value %= around;
  }
  all_value
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:02}:{:02}", &self.hours, &self.minutes)
  }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}:{}", self.hours, self.minutes)
    }
}