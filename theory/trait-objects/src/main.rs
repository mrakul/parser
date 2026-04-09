trait Shape {
    fn draw(&self);
}

struct Circle;
impl Shape for Circle {
    fn draw(&self) { println!("Drawing a circle"); }
}

struct Square;
impl Shape for Square {
    fn draw(&self) { println!("Drawing a square"); }
}

// Тип известен на этапе компиляции (создадутся две функции)
fn draw_static<T: Shape>(s: T) {
    s.draw();
} 

fn main() {
    // Статическая диспетчеризация
    draw_static(Circle);
    
    // Динамическая диспетчеризация
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle), Box::new(Square)];
    
    // Или так, но нужно инициализировать в таком случае переменные
    // let circle: Circle;
    // let square: Square;
    // let shapes: Vec<&dyn Shape> = vec![&circle, &square];
    
    for s in shapes {
        s.draw(); // вызывается через vtable
    }
}

