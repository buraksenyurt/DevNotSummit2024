# DevNot Summit 2024 - Rust ile Oyun Programlamada ECS Kullanımı

DevNot Developer Summit 2024 için oluşturulmuş repodur. Rust programlama dili ile oyun geliştirme konseptinde, **ECS** _(Entity Component System)_ kullanımına dair örnekler içermektedir.

## ECS Hakkında Genel Bilgiler

ECS çatısında oyundaki her nesne benzersiz bir tanımlayıcı ile işaretlenir ve bu bir Entity olarak ifade edilir. Entity'lere eklenebilecek verileri içeren datatype nesneleri de birer Component olarak tasarlanır. Sistemler belli bileşenlere sahip Entity setlerinin dolaşılması için kullanılır.

ECS, kodun yeniden kullanılabilirliğini _(Reusability)_ artırır ve veriyi davranışlardan _(Behavior)_ artırır.

## ECS ile OOP Arasındaki Farklar

- OOP tarafından kalıtım _(Inheritance)_ birinci sınıf vatandaş _(Citizen)_ ilen ECS'de composition'dır.
- OOP veriyi encapsulate etmeyi önerir, ECS ise Plain Old Data nesnelerini teşvik eder.
- ECS verileri davranışlardan ayırırken, OOP verileri davranışla birleştiren bir yol önerir.

## Composition over Inheritance İlkesi

Entity Component System, kalıtım yerine Composition over Inheritance yaklaşımını kullanır. Bir Entity tür hiyerarşisi yerine onunla ilişkili bileşenleri *(Component)* tarafından tanımlanır. Sistemler, istenen bileşenlere sahip Entity koleksiyonları üzerinde harket ederek çeşitli işlemler icra edebilir.Her ikisi arasındaki farkı yorumlamak için classic ve composition isimli Rust projelerinin kodlarına bakılabilir.

```shell
cargo run --bin classic

cargo run --bin composition
```

## Tarihçe

- Kayıtlara göre ECS'in ilk öncüsü 1998 yılında yayınlanan Thief: The Dark Project isimli oyundur. Bu oyunda kullanılan ECS motoru sonrasında devam oyununda ve System Shock 2 oyununda kullanılmış.
- 2007 yılında ECS sistemlerinin MMOG türünde kullanımı ile ilgili Adam Martin tarafından [detaylı bir yazı](https://t-machine.org/index.php/2007/09/03/entity-systems-are-the-future-of-mmog-development-part-1/) yayınlandı.
- 2015 yılında Apple, ECS'in bir uyarlamasını içeren ve iOS, macOS ve tvOS'larda oyun geliştirmek için kullanılan GameplayKit isimli bir framework yayınladı.
- 2018 yılında Sander Mertens [flecs](https://github.com/SanderMertens/flecs) isimli bir ECS Framework'ü oluşturdu. Bu framework C ve C++ için yapılmış bir uyarlamaydı.
- 2018 yılında Unity platformu da ECS'i kullanan bir demo yayınladı.