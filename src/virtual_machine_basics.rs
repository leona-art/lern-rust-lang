//! 仮想マシン概論
//! 
//! スタックベース仮想マシン


/// Rustでの実装
/// スタックベースの計算機のシンプルな実装を行う
/// 逆ポーランド記法での計算機
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

/// 標準入力からの読み込み
/// 標準出力からの読み込みは、テスト環境では難しいため、
/// 文字列を直接指定してテストする
mod sentence_2_3_read_from_stdin{

    fn read(input: &str) -> String {
        // 標準入力からの読み込みを模擬するために、引数として受け取った文字列を返す
        // 実際の標準入力からの読み込みはテスト環境では難しいため
        format!("Line: {:?}", input.split_whitespace().collect::<Vec<&str>>())
    }

    #[test]
    fn test_read_from_stdin() {
        let input="42 36 +";

        let line=read(input);
        assert_eq!(line, "Line: [\"42\", \"36\", \"+\"]");
    }
}

/// パースとコマンドの実行
/// 数値とコマンドに分解して計算を行う
mod sentence_2_4_parse_and_run_command{
    enum Operator{
        Add,
        Sub
    }
    enum Value{
        Number(i32),
        Operator(Operator),
    }

    fn parse(input: &str) -> Result<Vec<Value>, String> {

        input.split_whitespace().map(|s| {
            match s {
                "+" => Ok(Value::Operator(Operator::Add)),
                "-" => Ok(Value::Operator(Operator::Sub)),
                _ => s.parse::<i32>().map(Value::Number).map_err(|_| format!("Invalid number: {}", s)),
            }
        }).collect()
    }

    fn add(stack: &mut Vec<i32>) {
        if stack.len() < 2 {
            panic!("Stack underflow");
        }
        let rhs = stack.pop().unwrap();
        let lhs = stack.pop().unwrap();
        stack.push(lhs + rhs);
    }
    fn sub(stack: &mut Vec<i32>) {
        if stack.len() < 2 {
            panic!("Stack underflow");
        }
        let rhs = stack.pop().unwrap();
        let lhs = stack.pop().unwrap();
        stack.push(lhs - rhs);
    }

    fn calc(stack: &[Value]) -> Vec<i32> {
        let mut result = vec![];
        for value in stack {
            match value {
                Value::Number(n) => result.push(*n), 
                Value::Operator(Operator::Add) => add(&mut result), 
                Value::Operator(Operator::Sub) => sub(&mut result), 
            }
        }
        result
    }

    #[test]
    fn test_parse_and_run_command() {
        let inputs=[
            ("42 36 + 22 +",vec![100]),
            ("100 36 - 22 -",vec![42]),
            ("100 36 22 - + ",vec![114]),
            ("100 36 22 - + 10 -",vec![104]),
            ("100 36 22 - + 10 - 5 -",vec![99]),
            ("100 36 22 - + 10 - 5 - 2 +",vec![101]),
        ];

        for (input, expected) in inputs.iter() {
            let Ok(stack) = parse(input) else{
                panic!("Failed to parse input: {}", input);
            };
            
            let result = calc(&stack);
            assert_eq!(result, *expected);
        }

        
    }
}

/// ブロックとネスト構造
/// {}でトークンをグループ化する
mod sentence_2_5_block_and_nest{

    #[derive(Debug,PartialEq, Eq,Clone)]
    enum Op{
        Add,
        Sub
    }
    #[derive(Debug,PartialEq, Eq,Clone)]
    enum Value{
        Number(i32),
        Operator(Op),
        Block(Vec<Value>),
    }

    impl Value {
        fn display(&self) -> String {
            match self {
                Value::Number(n) => n.to_string(),
                Value::Operator(Op::Add) => "+".to_string(),
                Value::Operator(Op::Sub) => "-".to_string(),
                Value::Block(block) => format!("{{ {} }}", block.iter().map(|v| v.display()).collect::<Vec<_>>().join(" ")),
            }
        }
    }
    
    
    fn display(stack: &[Value]) -> String {
        stack.iter().map(|v| v.display()).collect::<Vec<_>>().join(" ")
    }

    fn parse_block<'a>(input: &'a[&'a str]) -> Result<(Vec<Value>, &'a [&'a str]), String> {
        let mut tokens = vec![];
        let mut words=input;
        while let Some((&word,mut rest))=words.split_first(){
            if word.is_empty(){
                break;
            }
            if word=="{"{
                let block:Vec<Value>;
                (block, rest) = parse_block(rest)?;
                tokens.push(Value::Block(block));
            }else if word=="}"{
                return Ok((tokens, rest));
            }else if let Ok(num)=word.parse::<i32>(){
                tokens.push(Value::Number(num));
            }else if word=="+"{
                tokens.push(Value::Operator(Op::Add));
            }else if word=="-"{
                tokens.push(Value::Operator(Op::Sub));
            }else{
                return Err(format!("Invalid token: {}", word));
            }
            words=rest;
        }
        Ok((tokens, words))
    }

    fn add(stack: &mut Vec<Value>) {
        if stack.len() < 2 {
            panic!("Stack underflow");
        }
        let rhs = stack.pop().unwrap();
        let lhs = stack.pop().unwrap();
        if let (Value::Number(lhs_num), Value::Number(rhs_num)) = (lhs, rhs) {
            stack.push(Value::Number(lhs_num + rhs_num));
        } else {
            panic!("Invalid operands for addition");
        }
    }
    fn sub(stack: &mut Vec<Value>) {
        if stack.len() < 2 {
            panic!("Stack underflow");
        }
        let rhs = stack.pop().unwrap();
        let lhs = stack.pop().unwrap();
        if let (Value::Number(lhs_num), Value::Number(rhs_num)) = (lhs, rhs) {
            stack.push(Value::Number(lhs_num - rhs_num));
        } else {
            panic!("Invalid operands for subtraction");
        }
    }
    fn tokenize(input: &str) -> Vec<&str> {
        input.split_whitespace().collect()
    }

    fn calc(stack: &[Value]) -> Vec<Value> {
        let mut result = vec![];
        for value in stack {
            match value {
                Value::Number(n) => result.push(Value::Number(*n)), 
                Value::Operator(Op::Add) => add(&mut result), 
                Value::Operator(Op::Sub) => sub(&mut result), 
                Value::Block(block) => result.push(Value::Block(block.clone())), // ブロックはそのままスタックに追加
                _ => {}
            }
        }
        result
    }

    

    #[test]
    fn test_block_and_nest() {
        let inputs=[
            ("1 2 + { 3 4 + } 5","3 { 3 4 + } 5"),
            ("{ 1 2 + } { 3 4 + }","{ 1 2 + } { 3 4 + }"),
            ("1 5 - { 2 3 + }","-4 { 2 3 + }"),
            ("5 6 + { 3 } 6 2 -","11 { 3 } 4"),
        ];
        for (input, expected) in inputs.iter() {
            let input = tokenize(input);
            let Ok((stack,_)) = parse_block(&input) else{
                panic!("Failed to parse input: {:?}", input);
            };
            let result = calc(&stack);
            assert_eq!(display(&result), *expected);
        }
    }
}

/// 2.6 if制御構文
/// { cond } { true_branch } { false_branch } if
mod sentence_2_6_if{

    /// 値
    #[derive(Debug,PartialEq, Eq,Clone)]
    enum Value{
        Number(i32),
        Operator(Op),
        Block(Vec<Value>)
    }

    impl Value{
        /// valueをblockとして取り出すヘルパー関数
        fn to_block(&self) -> Vec<Value> {
            match self {
                Self::Block(val) => val.clone(),
                _ => panic!("Not a block"),
            }
        }

        fn as_num(&self) -> i32 {
            match self {
                Self::Number(num) => *num,
                _ => panic!("Not a number"),
            }
        }
    }
    fn display(stack: &[Value]) -> String {
        stack.iter().map(|v| match v {
            Value::Number(n) => n.to_string(),
            Value::Operator(Op::Add) => "+".to_string(),
            Value::Operator(Op::Sub) => "-".to_string(),
            Value::Operator(Op::Mul) => "*".to_string(),
            Value::Operator(Op::Div) => "/".to_string(),
            Value::Operator(Op::If) => "if".to_string(),
            Value::Block(block) => format!("{{ {} }}", block.iter().map(|v| display(&v.to_block())).collect::<Vec<_>>().join(" ")),
        }).collect::<Vec<_>>().join(" ")
    }

    /// 演算子の定義
    #[derive(Debug,PartialEq, Eq,Clone)]
    enum Op{
        Add,
        Sub,
        Mul,
        Div,
        If,
    }

    impl TryFrom<&str> for Op {
        type Error = String;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "+" => Ok(Op::Add),
                "-" => Ok(Op::Sub),
                "*" => Ok(Op::Mul),
                "/" => Ok(Op::Div),
                "if" => Ok(Op::If),
                _ => Err(format!("Invalid operator: {}", value)),
            }
        }
    }

    fn add(stack: &mut Vec<Value>) {
        if stack.len() < 2 {
            panic!("Stack underflow");
        }
        let rhs = stack.pop().unwrap();
        let lhs = stack.pop().unwrap();
        if let (Value::Number(lhs_num), Value::Number(rhs_num)) = (lhs, rhs) {
            stack.push(Value::Number(lhs_num + rhs_num));
        } else {
            panic!("Invalid operands for addition");
        }
    }
    fn sub(stack: &mut Vec<Value>) {
        if stack.len() < 2 {
            panic!("Stack underflow");
        }
        let rhs = stack.pop().unwrap();
        let lhs = stack.pop().unwrap();
        if let (Value::Number(lhs_num), Value::Number(rhs_num)) = (lhs, rhs) {
            stack.push(Value::Number(lhs_num - rhs_num));
        } else {
            panic!("Invalid operands for subtraction");
        }
    }
    fn mul(stack: &mut Vec<Value>) {
        if stack.len() < 2 {
            panic!("Stack underflow");
        }
        let rhs = stack.pop().unwrap();
        let lhs = stack.pop().unwrap();
        if let (Value::Number(lhs_num), Value::Number(rhs_num)) = (lhs, rhs) {
            stack.push(Value::Number(lhs_num * rhs_num));
        } else {
            panic!("Invalid operands for multiplication");
        }
    }
    fn div(stack: &mut Vec<Value>) {
        if stack.len() < 2 {
            panic!("Stack underflow");
        }
        let rhs = stack.pop().unwrap();
        let lhs = stack.pop().unwrap();
        if let (Value::Number(lhs_num), Value::Number(rhs_num)) = (lhs, rhs) {
            if rhs_num == 0 {
                panic!("Division by zero");
            }
            stack.push(Value::Number(lhs_num / rhs_num));
        } else {
            panic!("Invalid operands for division");
        }
    }
    fn op_if(stack: &mut Vec<Value>) {
        if stack.len() < 3 {
            panic!("Stack underflow");
        }
        let false_branch = stack.pop().unwrap().to_block();
        let true_branch = stack.pop().unwrap().to_block();
        let cond = stack.pop().unwrap().to_block();

        for code in cond {
            eval(code, stack);
        }

        let cond_result = stack.pop().unwrap().as_num();
        if cond_result != 0 {
            for code in true_branch {
                eval(code, stack);
            }
        } else {
            for code in false_branch {
                eval(code, stack);
            }
        }
    }

    /// 値の評価関数
    fn eval(code:Value,stack:&mut Vec<Value>){
        match code {
            Value::Operator(Op::Add)=>add(stack),
            Value::Operator(Op::Sub)=>sub(stack),
            Value::Operator(Op::Mul)=>mul(stack),
            Value::Operator(Op::Div)=>div(stack),
            Value::Operator(Op::If)=>op_if(stack),
            _=>stack.push(code.clone()),
        }
    }

    /// スタックの評価関数
    fn parse<'a>(input: &'a str) -> Result<Vec<Value>, String> {
        let mut stack = Vec::new();
        let input=input.split_whitespace().collect::<Vec<_>>();
        let mut words=&input[..];

        while let Some((&word,mut rest))=words.split_first(){
            match word {
                w if w.is_empty() => break,
                "{" => {
                    let block;
                    (block, rest) = parse_block(rest)?;
                    stack.push(Value::Block(block));
                },
                _ => {
                    let code=if let Ok(num)=word.parse::<i32>(){
                        Value::Number(num)
                    }else if let Ok(op)=Op::try_from(word){
                        Value::Operator(op)
                    }else{
                        return Err(format!("Invalid token: {}", word));
                    };
                    eval(code, &mut stack);
                }
            }
            words=rest;
        }
        Ok(stack)
    }

    fn parse_block<'a>(input: &'a[&'a str]) -> Result<(Vec<Value>, &'a [&'a str]), String> {
        let mut tokens = vec![];
        let mut words=input;
        while let Some((&word,mut rest))=words.split_first(){
            if word.is_empty(){
                break;
            }
            if word=="{"{
                let block:Vec<Value>;
                (block, rest) = parse_block(rest)?;
                tokens.push(Value::Block(block));
            }else if word=="}"{
                return Ok((tokens, rest));
            }else if let Ok(num)=word.parse::<i32>(){
                tokens.push(Value::Number(num));
            }else if word=="+"{
                tokens.push(Value::Operator(Op::Add));
            }else if word=="-"{
                tokens.push(Value::Operator(Op::Sub));
            }else if word=="*"{
                tokens.push(Value::Operator(Op::Mul));
            }else if word=="/"{
                tokens.push(Value::Operator(Op::Div));
            }else if word=="if"{
                tokens.push(Value::Operator(Op::If));
            }else{
                // 無効なトークンの場合はエラーを返す
                return Err(format!("Invalid token: {}", word));
            }
            words=rest;
        }
        // パースされたトークンと残りの入力を返す
        Ok((tokens, words))
    }

    /// ifテスト関数
    /// 条件式とtrue/falseブランチの実行をテストする
    #[test]
    fn test_if() {
        let inputs=[
            // 条件が0になる場合（1-1=0）、falseブランチが実行される
            ("{ 1 -1 + } { 100 } { -100 } if","-100"),
            // 条件が非0になる場合（1+1=2）、trueブランチが実行される
            ("{ 1 1 + } { 100 } { -100 } if","100"),
        ];
        for (input, expected) in inputs.iter() {
            let Ok(result) = parse(input)else{
                panic!("Failed to parse input: {}", input);
            };
            assert_eq!(display(&result), *expected);
        }
    }
}

mod sentence_2_7_define_variable{
    use std::collections::HashMap;
    #[derive(Debug,PartialEq, Eq,Clone)]
    enum Op{
        Add,
        Sub,
        Mul,
        Div,
        If,
    }
    #[derive(Debug,PartialEq, Eq,Clone)]
    enum Value<'src>{
        Number(i32),
        Operator(Op),
        Block(Vec<Value<'src>>),
        Symbol(&'src str),
    }

    struct vm<'src>{
        stack: Vec<Value<'src>>,
        variables: HashMap<&'src str, Value<'src>>,
    }
}