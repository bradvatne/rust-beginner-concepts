struct CustomArray<T> {
    elements: Vec<T>,
}

impl<T> CustomArray<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Self { elements }
    }
}

pub fn main() {
    let test: CustomArray<i32> = CustomArray::new(vec![1, 2, 3]);
    let other: CustomArray<String> =
        CustomArray::new(vec![String::from("Hello"), String::from("This")]);
    for x in &test.elements {
        println!("{:?}", x)
    }
    for x in &other.elements {
        println!("{:?}", x)
    }
}
