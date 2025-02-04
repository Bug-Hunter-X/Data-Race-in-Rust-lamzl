use std::sync::Mutex;

fn main() {
    let x = Mutex::new(5);
    {
        let mut y = x.lock().unwrap();
        *y = 10;
    }
    {
        let mut z = x.lock().unwrap();
        *z = 15;
    }
    println!("x = {}", *x.lock().unwrap());
}