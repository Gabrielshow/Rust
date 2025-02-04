fn largest_number_in_list(list: &i32[]) -> i32 {
    let mut largest = &list[0];

    for &item in list.iter() {
        if &item > largest {
            largest = item;
        }
    }
    largest
}

// making this generic will look like this
// this code won't compile though, becuase not all types have 
// std::cmp::partialOrd trait implemented on them
fn largest_number<T: PartialOrd + Copy>(list: &T[]) -> T {
    let mut largest = &list[0];

     for &item in list.iter() {
        if &item > largest {
            largest = item;
        }
    }
    largest
}

fn return_largest_number< T: PartialOrd>(list:&T[]) -> &T {
    let mut largest = &list[0];

    for &item in list.iter() {
        if &item > largest {
            largest = item;

        }
    }
    &largest
} 

// you can also use generic types in struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> T {
            &self.x
    }
}

// now both instance will have the same type
// if you wanted to use/accommodate for different types you can use different generics
struct revisedPoint<T, U> {
    x: T,
    y: U,
}

enum Options<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

