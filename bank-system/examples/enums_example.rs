enum OuterWalletTouch {
    ContractUpdate{change: i32, reason: String, source: String}, // как структура
    P2pIncome(u32, String), // как кортеж
}
fn main () {
    // 4 записи различного типа
    let touches = vec![ 
                        OuterWalletTouch::P2pIncome(100, "Eisenhorn".into()),
                        OuterWalletTouch::ContractUpdate{
                                change: 200,
                                reason: "deposit".into(),
                                source: "Goldman Sachs".into()},
                        OuterWalletTouch::P2pIncome(0,   "Ravenor".into()),
                        OuterWalletTouch::ContractUpdate{
                                change: -100,
                                reason: "credit".into(),
                                source: "Horns and Hooves".into()
                            },];

    // Пустой вектор String'ов
    let mut sources = vec![];
    
    for touch in touches {
        // match по ссылке
        match &touch { 
            // Обработка первого типа
            OuterWalletTouch::ContractUpdate{change, source, reason}
                // Здесь:
                // 1. однострочная ветка match
                //    (все ветки должны возвращать одно и то же,
                //     здесь не возвращается ничего, то есть `()`)
                // 2. сырые r#""# -строки, где не нужно экранировать
                //    (r##""##, если может встретиться `"#`)
                // 3. выравнивание строк в println! (в format! тоже работает)
                //    через :4, знак > задаёт 'направление' выравнивания
                => println!( r#"From {:>16} change: {:4} (reason: "{:7}")"#,
                             source, change, reason ),

            // (!) С помощью if можно задать специальную ветку для некоторых значений enum
            OuterWalletTouch::P2pIncome(update, source) 
                if *update == 0 => println!("     {:>16} checked wallet is reachable", source),
            
            // Кроме однострочников, можно использовать {}-блоки
            OuterWalletTouch::P2pIncome(update, source) => {
                println!("From {:>16} change: {:4}, p2p", source, update);
            },
            // Паттерн "всё остальное"
            // (в этом примере будет вызывать warn "unreachable pattern",
            //  так как мы все ветки уже покрыли)
            _ => println!("unknown"),
        }

        // match по значению
        sources.push(match touch { 
            // Здесь:
            // 1. _ можно привязывать и к отдельным полям
            // 2. Поля можно 'переименовывать' (здесь: `source: my_source`)
            // 3. Паттерны можно объединять через |

            // То есть равен или первому, или второму типу, возвращается my_source, переименованный source
            // Остальные поля игнорируются
              OuterWalletTouch::ContractUpdate{change: _, reason: _, source: my_source} 
            | OuterWalletTouch::P2pIncome(_, my_source)
                => my_source
        });
    }
    println!("\nall sources: {:?}", sources);
}

// match может быть неудобен для проверки наличия конкретного варианта в перемещении перечислении. 
// Для этой задачи в Rust есть макрос matches!, который возвращает bool. 
// Его можно использовать так: 
//  if matches!(touch, OuterWalletTouch::ContractUpdate{change: 32, reason: _, ..}) {println!("matches!");}