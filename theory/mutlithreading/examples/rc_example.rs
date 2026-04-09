use std::rc::Rc;

#[derive(Debug)]
struct HolderV1<'a> {
    r: &'a u32,
}
#[derive(Debug)]
struct HolderV2 {
    r: Rc<u32>,
}
// Нельзя
// fn make_v1<'a>() -> (u32, HolderV1<'a>) {
//    let value = 42u32;
//    // Нельзя вернуть ссылку на локальную переменную
//    let holder = HolderV1{r: &value};
//    (value, holder)
// }
// Можно и нужно!
fn make_v2() -> (Rc<u32>, HolderV2) {
    let value = Rc::new(42u32);
    let holder = HolderV2{r: value.clone()};
    (value, holder)
}

fn main() {
    println!("{:?}", make_v2());
} 