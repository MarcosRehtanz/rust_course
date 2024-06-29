#[derive(Debug)]
struct KeyPress(String, char);
#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}
#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEKeys(KeyPress),
    WEClick(MouseClick),
}
// enum WebEvent {
//     WELoad,
//     WEKeys(String, char),
//     WEClick {x:i64, y:i64}
// }

fn fn_enum() {
    //Enum

    //load
    //we_load
    let we_load = WebEvent::WELoad(true);
    if let WebEvent::WELoad(loaded) = we_load {
        println!("Page loaded: {}", loaded);
    }
    //click
    let click = MouseClick { x: 100, y: 250 };
    println!("MouseClick: {}, {}", click.x, click.y);
    //we_click
    let we_click = WebEvent::WEClick(click);
    if let WebEvent::WEClick(MouseClick { x, y }) = we_click {
        println!("Mouse clicked at: ({}, {})", x, y);
    }
    //keys
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("KeyPress: {}, {}", keys.0, keys.1);
    //we_key
    let we_key = WebEvent::WEKeys(keys);
    /*
       TODO => in this tuple we are using ref for usi we_key later
    */
    if let WebEvent::WEKeys(KeyPress(ref combo, key)) = we_key {
        println!("Key combination pressed: {}{}", combo, key);
    }

    println!(
        "Load: {:#?}, Click: {:#?}, Key: {:#?}",
        we_load, we_click, we_key
    );
}

fn fn_struct() {
    // Struct
    // classic
    struct Student {
        name: String,
        level: u8,
        remote: bool,
    }
    // tuple
    struct Grades(char, char, char, char, f32);
    // unit
    // struct Unit;

    //? use classic struct
    let user_1 = Student {
        name: String::from("Constance Sharia"),
        remote: true,
        level: 2,
    };
    //? use tuple struct
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
    );

    struct Person {
        name: String,
    }
    let person: Person = Person {
        name: String::from("Juan"),
    };

    println!("Hello, world! I'am {}", person.name);
}

fn fn_tuple() {
    // Tuple
    let tuple_e = ('E', 5i8, true);
    println!(
        "char: {}, i8: {}, bool: {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );
}

fn fn_goodbye(message: &str) {
    println!("\n{}", message)
}

enum ReturnDuplicate {
    Int(u8),
    Str(&'static str),
}
fn fn_op_duplicate(num: u8) -> ReturnDuplicate {
    if num > 127 {
        return ReturnDuplicate::Str("Can not duplicate for number bigger than 127");
    } else {
        return ReturnDuplicate::Int(num * 2);
    }
}
fn fn_math(num: u8) {
    let result = fn_op_duplicate(num);
    match result {
        ReturnDuplicate::Int(value) => println!("\nfn_op_duplicate({}): {}", 175, value),
        ReturnDuplicate::Str(value) => println!("\nfn_op_duplicate({}): {}", 175, value),
    }
}

struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

impl Car {
    fn get_transmission(&self) -> &'static str {
        match self.transmission {
            Transmission::Automatic => "Automatic",
            Transmission::Manual => "Manual",
            Transmission::SemiAuto => "SemiAuto",
        }
    }
}

fn fn_create_car() {
    let car = Car {
        color: String::from("blue"),
        convertible: true,
        mileage: 1997,
        transmission: Transmission::Manual,
    };

    println!(
        "\nColor: {}, \nConvertible: {}, \nTransmission: {}, \nMileage: {} \n",
        car.color,
        car.convertible,
        car.get_transmission(),
        car.mileage
    )
}

fn main() {
    // let mut a_number;
    // let a_word = "Ten";
    // a_number = 10;
    // a_number = 12;

    fn_enum();
    fn_struct();
    fn_tuple();
    fn_goodbye("Goog bye Sr!");

    fn_math(175);
    fn_math(127);

    fn_create_car()
}
