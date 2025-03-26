//! 仮想マシン概論
//! 
//! スタックベース仮想マシン

fn push(stack: &mut Vec<i32>, value: i32) {
    stack.push(value);
}

fn add(stack: &mut Vec<i32>) {
    if stack.len() < 2 {
        panic!("Stack underflow");
    }
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    stack.push(lhs + rhs);
}

#[test]
fn test_stack_vm() {
    let mut stack:Vec<i32> = vec![];

    // 空のスタックを用意する
    assert_eq!(stack, vec![]);

    // スタックに値をプッシュする
    push(&mut stack, 42);
    assert_eq!(stack, vec![42]);

    // スタックに値をプッシュする
    push(&mut stack, 36);
    assert_eq!(stack, vec![42, 36]);

    // スタックの値を加算する
    add(&mut stack);
    assert_eq!(stack,vec![78]);

    push(&mut stack, 22);
    add(&mut stack);
    assert_eq!(stack, vec![100]);
}


