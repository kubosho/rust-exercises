/*
ゴール：
1. データをコピーしなくても、greetを呼び出せるようにしてください
2. "Hello dear rustaceans"ではなく、"Hello rustaceans"となるように、
   2回目のgreetの呼び出しを行なってください
3. 2をスライスを使って実現してください
*/

fn main() {
    let name = format!("dear rustaceans");
    greet(&name);
    greet(&name[5..]);
}

fn greet(name: &str) {
    println!("Hello {}", name);
}

