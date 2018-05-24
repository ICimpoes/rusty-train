fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = max(&number_list);
    println!("The max number is {:?}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = max(&number_list);
    println!("The max number is {:?}", result);

    let empty: Vec<i32> = vec![];
    let result: Option<&i32> = max(&empty);
    println!("The max number is {:?}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = max(&char_list);
    println!("The largest char is {:?}", result);
}

fn max<T: PartialOrd>(l: &[T]) -> Option<&T> {
    let mut m = l.get(0)?;

    for item in l.iter() {
        if item > m {
            m = item
        }
    }
    Some(m)
}

fn max_ch(list: &[char]) -> Option<char> {
    let mut largest = *list.get(0)?;

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}