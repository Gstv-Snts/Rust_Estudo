use std::thread;

fn main() {
    let mut list = vec![1, 3, 56, 7, 7, 2, 342, 5];
    let times_two = |mut num: usize| {
        num = num * 2;
        println!("{}", num)
    };
    let mut add_one = || list.push(1);
    add_one();
    let first = || println!("{}", list[0]);
    first();
    times_two(30);
    thread::spawn(move || println!("{:?}", list))
        .join()
        .unwrap();
}
