trait Sensor {
    fn read_event(&self);
}

struct TemperatureSensor;
struct DoorSensor;

impl Sensor for TemperatureSensor {
    fn read_event(&self) {
        println!("Температура: 23°C");
    }
}

impl Sensor for DoorSensor {
    fn read_event(&self) {
        println!("Дверь открыта!");
    }
}

fn main() {
    // Два разных типа сенсоров в векторе
    let sensors: Vec<Box<dyn Sensor>> = vec![
        Box::new(TemperatureSensor),
        Box::new(DoorSensor),
    ];

    for s in sensors {
        s.read_event(); // вызывается через vtable
    }
} 