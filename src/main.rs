use std::fmt::Debug;

fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();

    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    arr
}

enum TrafficLight {
    Green,
    Yellow,
    Red,
}

trait TrafficLightDuration {
    fn duration(&self) -> u8;
}

impl TrafficLightDuration for TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            Self::Green => 30,
            Self::Red => 20,
            Self::Yellow => 10,
        }
    }
}

fn safe_number(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;

    for &num in nums {
        match sum.checked_add(num) {
            Some(result) => sum = result,
            None => return None,
        }
    }

    Some(sum)
}


trait AreaCalculable {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    length: f64,
    width: f64,
}

#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64,
}

impl AreaCalculable for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl AreaCalculable for Square {
    fn area(&self) -> f64 {
        self.width * self.length
    }
}

impl AreaCalculable for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

fn print_area<T: AreaCalculable + Debug>(shape: &T) {
    println!("{:?} area: {}", shape, shape.area());
}

fn main() {
    println!("Hello, world!");
    println!("\n===================== Bubble Sort ================================\n");
    let unsorted_array = vec![50, 32, -8, 104, 288, 1, -32, 44];
    println!("original array: {:?}", unsorted_array);
    let sorted_array = bubble_sort(unsorted_array);
    println!("sorted array: {:?}", sorted_array);

    println!("\n===================== Traffic Light ================================\n");

    let green_light = TrafficLight::Green;
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;

    println!("Green light duration: {} seconds", green_light.duration());
    println!("Red light duration: {} seconds", red_light.duration());
    println!("Yellow light duration: {} seconds", yellow_light.duration());

    println!("\n===================== Safe Number ================================\n");

    let numbers = vec![50, 32, 8, 104, 288, 1, 32, 44];
    println!("Sum: {:?}", safe_number(&numbers).unwrap());
    let numbers_overflow = vec![50, 32, 8, 104, 288, 1, 32, 44, u32::MAX];
    println!("Sum: {:?}", safe_number(&numbers_overflow));

    println!("\n===================== Print Area ================================\n");
    
    let circle = Circle { radius: 6.6 };
    print_area(&circle);

    let triangle = Triangle {
        base: 34.5,
        height: 22.9,
    };
    print_area(&triangle);

    let square = Square {
        width: 22.5,
        length: 76.4,
    };
    print_area(&square);
}
