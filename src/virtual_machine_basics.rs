//! 仮想マシン概論
//! 
//! スタックベース仮想マシン

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
    assert_eq!(stack,vec![78])


}


