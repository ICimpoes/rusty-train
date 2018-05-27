fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = max(&number_list);
    println!("The max number is {:?}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = max(&number_list);
    println!("The max number is {:?}", result);

    let empty: Vec<i32> = vec![];
    let result: Option<i32> = max(&empty);
    println!("The max number is {:?}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = max(&char_list);
    println!("The largest char is {:?}", result);


    //
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?} {:?}", integer.x(), float);
    println!("{:?}", float.distance_from_origin());

    let float = Point2 { x: "1", y: 4.0 };
    println!("{:?} {:?}", integer, float);

    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 0 };
    let p3 = Point { x: -1, y: 0 };

    let v = vec![p1, p2, p3];

    println!("{:?}", max(&v))
}

fn max<T: PartialOrd + Copy>(l: &[T]) -> Option<T> {
    let mut m = *l.get(0)?;

    for &item in l.iter() {
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

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T: Clone> Clone for Point<T> {
    fn clone(&self) -> Point<T> {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T: Copy> Copy for Point<T> {}


use std::cmp::Ordering;

impl<T: PartialOrd> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Point<T>) -> Option<Ordering> {
        let mut m: Option<Ordering> = Some(Ordering::Equal);
        if &self.x > &other.x {
            m = Some(Ordering::Greater);
        }
        if &self.x < &other.x {
            m = Some(Ordering::Less);
        }

        m
    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Point<T>) -> bool {
        &self.x == &other.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}