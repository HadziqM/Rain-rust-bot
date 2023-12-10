use std::time::SystemTime;

pub struct MyTime;
impl MyTime {
    pub fn now()-> i64{
        i64::try_from(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()).unwrap()
    }
    pub fn elapsed(el:i64)-> i64{
        MyTime::now() + el
    }
}
