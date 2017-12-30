// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/dining-philosophers.html

// �\����
struct Philosopher {
   name: String,
}

// Philosopher �̒�`
impl Philosopher {
   // �֘A�֐��̒�`
   // str �̓v���~�e�B�u�^�A String �͕W�����C�u����
   // str �� u8 �̃X���C�X�ŌŒ蒷�A String �� u8 �̃x�N�^�ŉϒ�
   fn new(name: &str) -> Philosopher {
       // �Ō�̎��������I�ɖ߂�l�ƂȂ�
       Philosopher {
           // to_string �Ŏ��̃R�s�[���s���Ă���
           name: name.to_string(),
       }
   }
}

fn main() {
   let p1 = Philosopher::new("Judith Butler");
   let p2 = Philosopher::new("Gilles Deleuze");
   let p3 = Philosopher::new("Karl Marx");
   let p4 = Philosopher::new("Emma Goldman");
   let p5 = Philosopher::new("Michel Foucault");
}
