fn main() {
    print!("@{:.1}", get_beats(&time::now_utc()));
}

const SECONDS_TO_BEATS_FACTOR : f64 = 86.4;

fn get_beats(utctime: &time::Tm) -> f64 {
    let mut value : f64 = 0.0;
    value += f64::from(utctime.tm_sec);
    value += f64::from(utctime.tm_min * 60);
    value += f64::from(((utctime.tm_hour + 1) %24) * 3600);
    return value / SECONDS_TO_BEATS_FACTOR;
}

#[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn get_beats_should_return_correct_values(){
      let mut test_time : time::Tm = time::now_utc();

      test_time.tm_sec = 0;
      test_time.tm_min = 0;
      test_time.tm_hour = 23;
      
      assert_eq!(0.0, get_beats(&test_time));

      test_time.tm_sec = 59;
      test_time.tm_min = 59;
      test_time.tm_hour = 22;
      
      assert_eq!(9999.0, f64::floor(get_beats(&test_time)*10.0));

      test_time.tm_sec = 0;
      test_time.tm_min = 0;
      test_time.tm_hour = 5;
      
      assert_eq!(250.0, f64::ceil(get_beats(&test_time)));

      test_time.tm_sec = 0;
      test_time.tm_min = 0;
      test_time.tm_hour = 11;
      
      assert_eq!(500.0, f64::ceil(get_beats(&test_time)));

      test_time.tm_sec = 0;
      test_time.tm_min = 0;
      test_time.tm_hour = 17;
      
      assert_eq!(750.0, f64::ceil(get_beats(&test_time)));
  }
}
