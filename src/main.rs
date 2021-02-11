extern {
    fn receive_time64(sec: i64, nsec: i32);
    fn ret_p() -> *const libc::timespec;
}

fn send_and_retrieve() {
    let sec = 3;
    let nsec = 4;

    let t: libc::timespec;
    unsafe {
        receive_time64(sec, nsec);
        t = *ret_p();
    }

    println!("value: expected actual_result");
    println!("sec: {} {}", sec, t.tv_sec);
    println!("nsec: {} {}", nsec, t.tv_nsec);
}

fn main() {
    send_and_retrieve();
}
