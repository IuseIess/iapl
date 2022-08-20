use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please input a string to process!");
        std::process::exit(1);
    }
    let s = &args[1];
    let mut windows = s.chars();
    let mut flag: bool = false;
    let mut result: Vec<char> = vec![];
    while let Some(v) = windows.next() {
        if flag == true {
            match v {
                '[' => result.push('←'),
                '{' => result.push('⍞'),
                '-' => result.push('×'),
                '=' => result.push('÷'),
                '+' => result.push('⌹'),
                '8' => result.push('≠'),
                '*' => result.push('⍟'),
                'o' => result.push('○'),
                'O' => result.push('⍥'),
                's' => result.push('⌈'),
                'd' => result.push('⌊'),
                'b' => result.push('⊥'),
                'n' => result.push('⊤'),
                '|' => result.push('⊣'),
                'B' => result.push('⊢'), // alternative to '\\'
                '4' => result.push('≤'),
                '6' => result.push('≥'),
                ':' => result.push('≡'),
                ';' => result.push('⍎'),
                '"' => result.push('≢'),
                '9' => result.push('∨'),
                ')' => result.push('⍲'),
                '(' => result.push('⍱'),
                'y' => result.push('↑'),
                'u' => result.push('↓'),
                'z' => result.push('⊂'),
                'Z' => result.push('⊆'),
                'x' => result.push('⊃'),
                'L' => result.push('⌷'),
                'l' => result.push('⎕'),
                '$' => result.push('⍋'),
                '#' => result.push('⍒'),
                'i' => result.push('⍳'),
                'I' => result.push('⍸'),
                'e' => result.push('∊'),
                'E' => result.push('⍷'),
                'v' => result.push('∪'),
                'c' => result.push('∩'),
                't' => result.push('~'),
                'T' => result.push('⍨'),
                '/' => result.push('⌿'),
                '.' => result.push('⍀'),
                '>' => result.push('⍙'),
                '<' => result.push('⍪'),
                'r' => result.push('⍴'),
                '%' => result.push('⌽'),
                '&' => result.push('⊖'),
                '^' => result.push('⍉'),
                '1' => result.push('¨'),
                'P' => result.push('⍣'),
                'j' => result.push('J'),
                'J' => result.push('⍤'),
                '?' => result.push('⍠'),
                'K' => result.push('⌸'),
                '~' => result.push('⌺'),
                '!' => result.push('⌶'),
                '\'' => result.push('⍕'),
                '`' => result.push('⋄'),
                ',' => result.push('⍝'),
                ']' => result.push('→'),
                'w' => result.push('⍵'),
                'a' => result.push('⍺'),
                'g' => result.push('∇'),
                '2' => result.push('¯'),
                '}' => result.push('⍬'),
                x => result.push(x),
            }
            flag = false;
        } else {
            match v {
                '\\' => {
                    flag = true;
                    continue;
                },
                x => result.push(x),
            }
        }
    }
    println!("{}", result.into_iter().collect::<String>());
}
