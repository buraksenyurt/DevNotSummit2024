# DevNot Summit 2024 - Rust ile Oyun Programlamada ECS Kullanımı

DevNot Developer Summit 2024 için oluşturulmuş repodur. Rust programlama dili ile oyun geliştirme konseptinde, **ECS** _(Entity Component System)_ kullanımına dair örnekler içermektedir.

## ECS Hakkında Genel Bilgiler

ECS çatısında oyundaki her nesne benzersiz bir tanımlayıcı ile işaretlenir ve bu bir Entity olarak ifade edilir. Entity'lere eklenebilecek verileri içeren datatype nesneleri de birer Component olarak tasarlanır. Sistemler belli bileşenlere sahip Entity setlerinin dolaşılması için kullanılır.

ECS, kodun yeniden kullanılabilirliğini _(Reusability)_ artırır ve veriyi davranışlardan _(Behavior)_ artırır.

Örneğin Tower Defence tadındaki bir oyunu düşünelim. Entity ve Component ilişkilerini aşağıdaki gibi özetleyebiliriz.

```text
+----------------+----------+----------+-------------+
|   Components   |  Tower   |  Enemy   | Bullet      |
+----------------+----------+----------+-------------+
| Position       | (x, y)   | (x, y)   | (x, y)      |
| Health         |          | (hp)     |             |
| Damage         | (dmg)    |          | (dmg)       |
| Range          | (range)  |          |             |
| Velocity       |          | (vx, vy) | (vx, vy)    |
+----------------+----------+----------+-------------+
```

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

# ECS in Kullanıldığı Diğer Alanlar

- **Simülasyon Yazılımları :** ECS, karmaşık sistemlerin modellenmesi gereken simülasyon yazılımlarında kullanılabilir. Örneğin, trafik simülasyonlarını ele alalım. Arabalar ve yayalar birer Entity olarak düşünülebilir. Araçların konumları, hızları ve yönleri birer bileşen _(Component)_ olarak tasarlanabilir. Sistemler, çarpışma algılama ve rota planlama gibi işlevleri yürütebilir.
- **Robotik/IoT :** Robitik veya IoT sistemlerde bir cihazın parçalarını ve etkileşimlerini yönetmek için ECS'den yararlanılabilir. Örneğin bir robotun farklı uzuvları birer Entity olarak düşünülebilir. Kolları, sensörleri, ayakları vs. Yine bu nesnelerin konumları, anlık durumları birer bileşen olaran düşünülebilir. Sistemler bu parçaların koordinasyon ve kontrolünü yönetir ve gezinme, rota belirleme, metrik ölçümleyip durum tespiti yapma, çevre tarama ve basit görevleri etkinleştirir.
- **Data-Driven Mimariler :** Büyük verilerin _(Big Data)_ işlenmesi ve analizinde kullanılabilir. Veri akışları _(Data Streams)_ birer Entity olabilir, metadata ve transformation kuralları ise birer bileşen olarak düşünülebilir. Sistemler verileri bu kurallara göre işler ve analiz eder.
- **Sanal/Artırılmış Gerçeklik (VR/AR) :** Sanal ortamdaki nesneler birer Entity olarak temsil edebilir. Bu nesnelerin fiziksel özellikleri ve davranışları ise birer bileşen olarak düşünülebilir. Sistemler rendering, etkileşim ve gerçek hayat fizik ilkelerini işleyebilir.
- **UI Frameworks :** Bu tip bir framework içerisinde Button, Slider, CheckBox, TextBox gibi unsular birer Entity olarak düşünüldüğünde boyutları, renkleri, durumları vb unsurlar da bileşen olarak tesis edilebilir. Sistemler çeşitli bileşenlere sahip entity nesnelerinin render edilmesi veya kullanıcı ile etkileşimini yönetebilir.