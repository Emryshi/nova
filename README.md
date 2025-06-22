# NovaLang

NovaLang, modern, öğrenilebilir ve genişletilebilir bir programlama dilidir. Python ve JavaScript'ten ilham alır, eğitim ve ileri düzey programlama konseptlerini öğretmek için tasarlanmıştır.

## Özellikler
- Temiz ve okunabilir sözdizimi
- Değişkenler, fonksiyonlar, if-else, while döngüsü
- int, float, string, bool, null veri tipleri
- Kapsamlı hata yönetimi
- REPL ve dosya çalıştırıcı
- Genişletilebilir mimari (OOP, modül, try-catch, async/await eklenebilir)

## Kurulum
```sh
cargo build --release
```

## Kullanım
### REPL
```sh
cargo run
```

### Dosya Çalıştırma
```sh
cargo run -- example.nova
```

## NovaLang Sözdizimi Örneği
```novalang
let x = 10;
func square(n) {
    return n * n;
}
print(square(x));
```

## Dosya Yapısı
```
nova/
  ├── src/
  │   ├── token.rs
  │   ├── lexer.rs
  │   ├── parser.rs
  │   ├── ast.rs
  │   ├── semantic.rs
  │   ├── vm.rs
  │   ├── runtime.rs
  │   ├── error.rs
  │   └── main.rs
  ├── Cargo.toml
  ├── README.md
  └── example.nova
```

## Katkı
Genişletmek ve katkı sağlamak için PR gönderebilirsiniz. 