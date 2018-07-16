// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/guessing-game.html

// use std::io::stdin; なんてこともできる
use std::io;

// python でいう def が rust の fn
// c++ 同様 {} でスコープを指定。行端は ;
fn main()
{
   // println! はマクロ
   println!("Guess the number!");
   // let は束縛変数の定義。イミュータブルな変数を定義する
   // mut でミュータブルに変更可能
   // String は標準ライブラリが提供。String::new() はスタティックメソッド
   let mut guess = String::new();
   // read_line はメソッド。インスタンスがあるときだけ呼び出せる
   // & は参照。参照もデフォルトでイミュータブルなので mut が必要
   io::stdin().read_line(&mut guess)
   // read_line の戻り値は io::Result で expect() メソッドが定義されている
   // expect(message) が呼び出されると panic! が呼び出され終了する
   // expect を呼び出さないとコンパイラは警告を発する
              .expect("Failed to read line");
   // println! では {} で引数をうけとることができる
   println!("You guessed: {}", guess);
}

// extern crate で [dependencies] で定義したライブラリを使える
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
   println!("Guess the number!");
   // トレイト Rng をスコープに置くことでメソッド thread_rng, gen_range を使用できる
   let secret_number = rand::thread_rng().gen_range(1, 101);
   println!("The secret number is: {}", secret_number);

   println!("Please input your guess.");
   let mut guess = String::new();
   io::stdin().read_line(&mut guess)
       .expect("failed to read line");
   println!("You guessed: {}", guess);
}

extern crate rand;



use std::io;

use std::cmp::Ordering;

use rand::Rng;



fn main() {
   println!("Guess the number!");
   let secret_number = rand::thread_rng().gen_range(1, 101);
   println!("The secret number is: {}", secret_number);

   println!("Please input your guess.");
   let mut guess = String::new();
   io::stdin().read_line(&mut guess)
       .expect("failed to read line");
   println!("You guessed: {}", guess);

   // シャドーイングを使って値や型を再定義できる
   // trim で行末の \n を取り除く
   // parse は文字列から数値へ変換する
   let guess: u32 = guess.trim()
                         .parse()
                         .expect("Please type a number!");
   println!("You guessed: {}", guess);
   // cmp() 比較可能なもの全てに対して呼べる
   // Ordering は列挙型
   // match文は値を受け取った後、任意の条件で条件分岐を作れる
   match guess.cmp(&secret_number) {
       Ordering::Less    => println!("Too small!"),
       Ordering::Greater => println!("Too big!"),
       Ordering::Equal   => println!("You win!"),
   }
}

extern crate rand;



use std::io;

use std::cmp::Ordering;

use rand::Rng;



fn main() {
   println!("Guess the number!");

   let secret_number = rand::thread_rng().gen_range(1, 101);

   println!("The secret number is: {}", secret_number);

   // 無限ループの作成
   loop {
       println!("Please input your guess.");
       let mut guess = String::new();
       io::stdin().read_line(&mut guess)
           .expect("failed to read line");

       // parse が enum を返却。match文で Ok, Err 判定をする
       let guess: u32 = match guess.trim().parse() {
           Ok(num) => num,
           Err(_) => continue,
       };

       println!("You guessed: {}", guess);
       match guess.cmp(&secret_number) {
           Ordering::Less    => println!("Too small!"),
           Ordering::Greater => println!("Too big!"),
           Ordering::Equal   => {
               println!("You win!");
               break;
           }
       }
   }
}
