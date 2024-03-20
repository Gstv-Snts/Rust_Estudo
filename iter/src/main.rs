#[derive(PartialEq, Debug)]
struct Person {
    name: &'static str,
    age: usize,
}
fn main() {
    let v1: Vec<usize> = vec![1, 3, 5, 6];
    let v1_iter = v1.iter();
    for num in v1_iter {
        println!("{:?}", num);
    }
    let v1_iter = v1.iter();
    let total: usize = v1_iter.sum();
    println!("{:?}", total);
    let v1_map: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v1_map);
    let people = [
        Person { name: "aa", age: 1 },
        Person { name: "bb", age: 2 },
        Person { name: "cc", age: 3 },
    ];
    let people_filter: Vec<Person> = people
        .into_iter()
        .filter(|person| person.age != 2)
        .collect();
    println!("{:?}", people_filter);
}
