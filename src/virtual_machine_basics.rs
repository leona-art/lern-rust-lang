//! 仮想マシン概論
//! 
//! スタックベース仮想マシン


mod sentence_2_2_impl_rust{
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
}

mod sentence_2_3_read_from_stdin{

    fn read(input: &str) -> String {
        // 標準入力からの読み込みを模擬するために、引数として受け取った文字列を返す
        // 実際の標準入力からの読み込みはテスト環境では難しいため
        format!("Line: {:?}", input.split_whitespace().collect::<Vec<&str>>())
    }
    
    // なので、標準出力からの読み込みはテストしない
    // 代わりに、文字列を直接指定してテストする
    #[test]
    fn test_read_from_stdin() {
        let input="42 36 +";

        let line=read(input);
        assert_eq!(line, "Line: [\"42\", \"36\", \"+\"]");
    }
}
