use rand::Rng;
use std::cmp::Ordering;
use std::fmt;
use std::io;
#[derive(Debug)]
struct PointD {
    x: f64,
    y: f64,
}
impl fmt::Display for PointD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lng: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { "N" } else { "S" };
        let lng_c = if self.lng >= 0.0 { "E" } else { "W" };

        write!(
            f,
            "{}: {:.3}{} {:.3}{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lng.abs(),
            lng_c
        )
    }
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

struct RectPoints {
    x: i32,
    y: i32,
}

fn rect_area(pair: RectPoints) -> i32 {
    pair.x * pair.y
}

fn fizz_buzz() {
    let num_range = 1..=100;

    for n in num_range {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", n);
        }
    }
}

fn iteration() {
    let mut names = vec!["solomon", "tobi", "tobe"];

    // for name in names.iter(){
    //     match name {
    //         &"solomon" => println!("there's a rustacean among us"),
    //         _ => println!("hello {}", name)
    //     }
    // }

    // for name in names.into_iter(){
    //     match name {
    //         "solomon" => println!("this is solomon, za rustacean,"),
    //         _ => println!("these are his guys")
    //     }
    // }

    for name in names.iter_mut() {
        *name = match name {
            &mut "solomon" => "the first and the last",
            _ => "everyone",
        };
    }
    println!("names {:?}", names)
}

fn age() -> u32 {
    15
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn guessing_game() {
    println!("make a guess, choose a number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is: {secret_number}");

    loop {
        println!("choose your number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("guess is too high"),
            Ordering::Less => println!("guess is too low"),
            Ordering::Equal => {
                println!("Hooray, you have won");
                break;
            }
        }
    }
}

fn string_slice(s: &String) -> &str {
    let string = s.as_bytes();

    for (i, &item) in string.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


fn main() {
    println!("Hello, world!");
    let points = PointD { x: 23.0, y: 33.0 };
    println!("Debug: {:?}", points);

    let rectangle_area = rect_area(RectPoints { x: 12, y: 6 });
    println!("rectangle area: {}", rectangle_area);

    let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
    println!("{:?}", matrix);

    for city in [
        City {
            name: "Dublin",
            lat: 53.34778,
            lng: -6.259722,
        },
        City {
            name: "Morroco",
            lat: 59.95,
            lng: 10.75,
        },
        City {
            name: "Seychelles",
            lat: 49.25,
            lng: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city)
    }

    let elem = 23u8;

    let mut vec = Vec::new();
    vec.push(elem);

    println!("{:?}", vec);

    let arr = [1, -2, 6];
    let tuple_ppty = (1, 2, false);

    // destructuring arrays
    match arr {
        [first, middle @ .., third] => println!(
            "the first is {}, second is {:?}, third is {}",
            first, middle, third
        ),
    }

    // destructuring tuples
    match tuple_ppty {
        (first, second, ppty) => println!(
            "the first val: {}, the second val: {}, the third val is either {} or false",
            first, second, ppty
        ),
    }

    // enum temperature
    enum Temperature {
        Celcius(i32),
        Fahrenheit(i32),
    }

    let temperature = Temperature::Celcius(35);

    match temperature {
        Temperature::Celcius(t) if t > 30 => println!("the temperature {}C is higher than 30C", t),
        Temperature::Celcius(t) => println!("{}C the value is below 30C", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("the value {}F is greater than 86F", t),
        Temperature::Fahrenheit(t) => println!("the value {}F is lower than", t),
    }

    match age() {
        0 => println!("I have not celebrated my birthday yet"),
        n @ 1..=12 => println!("I am a child of the age of {:?}", n),
        n @ 13..=19 => println!("I am a child of the age of {:?}", n),
        n => println!("I am an old person of the age of {:?}", n),
    }

    let result = divide(23.0, 1.9);

    match result {
        Some(n) => println!("the result is {n}",),
        None => println!("cannot divide by zero"),
    }

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit");
            optional = None;
        } else {
            println!("i is {:?}. Try again", i);
            optional = Some(i + 1)
        }
    }

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit");
                    optional = None;
                } else {
                    println!("i is {:?}. Try again", i);
                    optional = Some(i + 1)
                }
            }
            _ => {
                break;
            }
        }
    }
}
