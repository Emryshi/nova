# NovaLang

NovaLang, modern, öğrenilebilir ve genişletilebilir bir programlama dilidir. Python ve JavaScript'ten ilham alır, eğitim ve ileri düzey programlama konseptlerini öğretmek için tasarlanmıştır.

## Özellikler
- Temiz ve okunabilir sözdizimi
- Değişkenler, fonksiyonlar, if-else, while döngüsü
- int, float, string, bool, null, list, map veri tipleri
- try-catch-finally ile kapsamlı hata yönetimi
- **Nesne Yönelimli Programlama:** class, inheritance, method, object
- **Async/Await:** Asenkron fonksiyonlar ve bekleme
- **Modül Sistemi:** import/export, standart kütüphane
- **Fonksiyonel programlama:** lambda, fonksiyonları değişkene atama
- **Performans:** Bytecode/VM altyapısı, hızlı environment
- REPL ve dosya çalıştırıcı

## Standart Kütüphane
- `math`: square, cube, abs
- `string`: upper, lower, length
- `file`: read, write
- `net`: get (httpGet)

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
import math;
import string;
import file;
import net;

class Animal {
    func speak() { print("..."); }
}
class Dog : Animal {
    func speak() { print("Hav!"); }
}
let d = Dog();
d.speak();

async func fetchData() {
    return net.get("http://example.com");
}
let data = await fetchData();
print(data);

try {
    let x = 1 / 0;
} catch (err) {
    print("Hata:", err);
} finally {
    print("Her zaman çalışır");
}

let s = "merhaba";
print(string.upper(s));
print(math.square(5));
print(file.read("test.txt"));
```

## Dosya Yapısı
```