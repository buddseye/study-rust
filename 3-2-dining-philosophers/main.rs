// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/dining-philosophers.html

// 構造体
struct Philosopher {
   name: String,
}

// Philosopher の定義
impl Philosopher {
   // 関連関数の定義
   // str はプリミティブ型、 String は標準ライブラリ
   // str は u8 のスライスで固定長、 String は u8 のベクタで可変長
   fn new(name: &str) -> Philosopher {
       // 最後の式が自動的に戻り値となる
       Philosopher {
           // to_string で実体コピーを行っている
           name: name.to_string(),
       }
   }

   fn eat(&self) {
       println!("{} is done eating.", self.name);
   }
}

fn main() {
    // ベクタ配列Vec<T>
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];
    for p in &philosophers {
        p.eat();
    }
}
