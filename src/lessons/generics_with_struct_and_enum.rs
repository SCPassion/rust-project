// using generics with struct and enum
fn generics_with_struct_and_enum() {
    // ======== generic struct ========
    let int_container = Container {value: 42};
    let str_container = Container {value: "Simon"};

    println!("{}", int_container.value);
    println!("{}", str_container.value);

    // generic enum
    let success_result: MagicResult<i32, String> = MagicResult::Success(42);
    let failure_result: MagicResult<i32, String> = MagicResult::Failure("It failed".to_string());

    match success_result {
        MagicResult::Success(value) => println!("Success: {}", value),
        MagicResult::Failure(error) => println!("Failure: {}", error),
    }

    match failure_result {
        MagicResult::Success(value) => println!("Success: {}", value),
        MagicResult::Failure(error) => println!("Failure: {}", error),
    }
    
    // ======== Using generics in associated types ========
    let mut honda = ElectricCar { battery_level: 42};
    let mut ford = GasCar { gas_level: 20};

    honda.refuel(58);
    ford.refuel(0.2);
}


// ======== generic struct ========
struct Container<T> {
    value: T,
}

//generic enum
enum MagicResult<T, E> {
    Success(T),
    Failure(E),
}


// ======== Using generics in associated types ========
trait Vehicle {
    // This fuel type is generic and will be pass to reful method to determine the type of fuel for input parameter
    // Associated type: associate a specific type with a trait, making it possible to use that type within the trait's methods. 
    type Fuel;

    fn refuel(&mut self, fule: Self::Fuel);
}

struct ElectricCar {
    battery_level: u32,
}

struct GasCar {
    gas_level: u32,
}

// Fuel type is u32
impl Vehicle for ElectricCar {
    type Fuel = u32;

    fn refuel(&mut self, charge: Self::Fuel) {
        self.battery_level += charge;
        println!("Refueled with {} units of electricity", charge);
        println!("Battery level: {}%", self.battery_level);
    }   
}

// Fuel type is f32
impl Vehicle for GasCar {
    type Fuel = f32;

    fn refuel(&mut self, gas: Self::Fuel) {
        self.gas_level += (gas * 100.0) as u32;
        println!("Refueled with {} units of gas", gas);
        println!("Gas level: {}%", self.gas_level);
    }
}


