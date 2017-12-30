// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/guessing-game.html

// use std::io::stdin; �Ȃ�Ă��Ƃ��ł���
use std::io;

// python �ł��� def �� rust �� fn
// c++ ���l {} �ŃX�R�[�v���w��B�s�[�� ;
fn main()
{
   // println! �̓}�N��
   println!("Guess the number!");
   // let �͑����ϐ��̒�`�B�C�~���[�^�u���ȕϐ����`����
   // mut �Ń~���[�^�u���ɕύX�\
   // String �͕W�����C�u�������񋟁BString::new() �̓X�^�e�B�b�N���\�b�h
   let mut guess = String::new();
   // read_line �̓��\�b�h�B�C���X�^���X������Ƃ������Ăяo����
   // & �͎Q�ƁB�Q�Ƃ��f�t�H���g�ŃC�~���[�^�u���Ȃ̂� mut ���K�v
   io::stdin().read_line(&mut guess)
   // read_line �̖߂�l�� io::Result �� expect() ���\�b�h����`����Ă���
   // expect(message) ���Ăяo������ panic! ���Ăяo����I������
   // expect ���Ăяo���Ȃ��ƃR���p�C���͌x���𔭂���
              .expect("Failed to read line");
   // println! �ł� {} �ň����������Ƃ邱�Ƃ��ł���
   println!("You guessed: {}", guess);
}

// extern crate �� [dependencies] �Œ�`�������C�u�������g����
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
   println!("Guess the number!");
   // �g���C�g Rng ���X�R�[�v�ɒu�����ƂŃ��\�b�h thread_rng, gen_range ���g�p�ł���
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

   // �V���h�[�C���O���g���Ēl��^���Ē�`�ł���
   // trim �ōs���� \n ����菜��
   // parse �͕����񂩂琔�l�֕ϊ�����
   let guess: u32 = guess.trim()
                         .parse()
                         .expect("Please type a number!");
   println!("You guessed: {}", guess);
   // cmp() ��r�\�Ȃ��̑S�Ăɑ΂��ČĂׂ�
   // Ordering �͗񋓌^
   // match���͒l���󂯎������A�C�ӂ̏����ŏ������������
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

   // �������[�v�̍쐬
   loop {
       println!("Please input your guess.");
       let mut guess = String::new();
       io::stdin().read_line(&mut guess)
           .expect("failed to read line");

       // parse �� enum ��ԋp�Bmatch���� Ok, Err ���������
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
