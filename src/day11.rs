#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    change_operation: Operation,
    change_value: Value,
    test_number: u64,
    true_test: usize,
    false_test: usize,
    business: u64,
}

#[derive(Debug, Clone)]
enum Operation {
    Multiply,
    Add,
}

#[derive(Debug, Clone)]
enum Value {
    Number(u64),
    Old,
}

fn game1(mut monkeys: Vec<Monkey>) -> u64 {
    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let mut throws: Vec<(usize, u64)> = Vec::new();

            let monkey = &mut monkeys[index];

            for item in monkey.items.iter_mut() {
                let modifier = match monkey.change_value {
                    Value::Number(value) => value,
                    Value::Old => *item,
                };

                match monkey.change_operation {
                    Operation::Multiply => *item *= modifier,
                    Operation::Add => *item += modifier,
                }

                *item /= 3;

                if *item % monkey.test_number == 0 {
                    throws.push((monkey.true_test, *item));
                } else {
                    throws.push((monkey.false_test, *item));
                }
                
                monkey.business += 1;
            }

            for (recipient, item) in throws.iter().rev() {
                monkeys[*recipient].items.push(*item);
            }

            monkeys[index].items.clear();
        }
    }

    monkeys.sort_by(|a, b| a.business.cmp(&b.business));
    monkeys.iter().map(|monkey| monkey.business ).rev().take(2).product::<u64>()
}

fn game2(mut monkeys: Vec<Monkey>) -> u64 {
    let absolute_limit: u64 = monkeys.iter().map(|monkey| monkey.test_number).product();

    for _ in 0..10000 {
        for index in 0..monkeys.len() {
            let mut throws: Vec<(usize, u64)> = Vec::new();

            let monkey = &mut monkeys[index];

            for item in monkey.items.iter_mut() {
                let modifier = match monkey.change_value {
                    Value::Number(value) => value,
                    Value::Old => *item,
                };

                match monkey.change_operation {
                    Operation::Multiply => *item *= modifier,
                    Operation::Add => *item += modifier,
                }

                *item %= absolute_limit;

                if *item % monkey.test_number == 0 {
                    throws.push((monkey.true_test, *item));
                } else {
                    throws.push((monkey.false_test, *item));
                }
                
                monkey.business += 1;
            }

            for (recipient, item) in throws.iter().rev() {
                monkeys[*recipient].items.push(*item);
            }

            monkeys[index].items.clear();
        }
    }

    monkeys.sort_by(|a, b| a.business.cmp(&b.business));
    monkeys.iter().map(|monkey| monkey.business ).rev().take(2).product::<u64>()
}

pub fn run(input: &str) {
    let monkeys: Vec<Monkey> = input.split("\n\n").map(|monkey_input| {
        let input: Vec<&str> = monkey_input.split('\n').collect();

        let items: Vec<u64> = input[1].split(' ').skip(4).map(|item| {
            let index = item.find(',').unwrap_or(item.len());
            item[0..index].parse::<u64>().unwrap()
        }).collect();
        let test_number: u64 = input[3].split(' ').last().unwrap().parse().unwrap();
        let true_test: usize = input[4].split(' ').last().unwrap().parse().unwrap();
        let false_test: usize = input[5].split(' ').last().unwrap().parse().unwrap();

        let operation_input: Vec<&str> = input[2].split(' ').skip(6).collect();

        let change_operation = match operation_input[0] {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => unreachable!(),
        };
        let change_value = match operation_input[1] {
            "old" => Value::Old,
            _ => Value::Number(operation_input[1].parse().unwrap()),
        };

        Monkey {items, change_operation, change_value, test_number, true_test, false_test, business: 0 }
    }).collect();

    let product1 = game1(monkeys.clone());
    let product2 = game2(monkeys.clone());
    
    println!("{}\n{}", product1, product2);
}
