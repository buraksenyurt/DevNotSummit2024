# DevNot Summit 2024 Rust ile Oyun Programlamada ECS Kullanımı

[DevNot Developer Summit 2024](https://summit.devnot.com/2024/) için oluşturulmuş repodur. Rust programlama dili ile oyun geliştirme konseptinde, **ECS** _(Entity Component System)_ kullanımına dair örnekler ve kısa notlar içermektedir.

- [İçerik](#devnot-summit-2024-rust-ile-oyun-programlamada-ecs-kullanımı)
  - [Projeler](#projeler)
  - [Tanım](#tanım)
  - [ECS Neden Gereklidir?](#ecs-neden-gereklidir)
  - [ECS Hakkında Genel Bilgiler](#ecs-hakkında-genel-bilgiler)
  - [Composition Over Inheritance](#composition-over-inheritance)
  - [ECS ile OOP Arasındaki Farklar](#ecs-ile-oop-arasındaki-farklar)
  - [Tarihçe](#tarihçe)
  - [ECS in Kullanıldığı Diğer Alanlar](#ecs-in-kullanıldığı-diğer-alanlar)
  - [Bevy ECS Hakkında](#bevy-ecs-hakkında)
  - [Kaynaklar](#kaynaklar)

## Projeler

| Proje Adı | Açıklama |
|-----------|----------|
| **classic** | Kalıtım (inheritance) tabanlı klasik OOP yaklaşımı ile Archer, Warrior, Healer karakterlerinin implementasyonu |
| **composition** | Composition over Inheritance prensibiyle bileşenler (Position, Health, Damage, vb.) kullanarak karakter tasarımı |
| **dream-catcher** | Bevy ile uzay gemisi kontrolü ve top hareketi içeren basit bir oyun prototipi |
| **game_with_bevy** | Bevy ECS kullanarak Player ve Enemy varlıklarının hareket ve hedef kontrolü simulasyonu |
| **game_without_bevy** | Bevy kullanmadan manuel struct'larla Player, Enemy ve Goal mekanizmasının implementasyonu |
| **hello_ecs** | Tower Defence tarzı oyun için Entity-Component-System yapısının modüler organizasyonu |
| **simple_ecs** | Basit bir ECS implementasyonu ile Health, Attack ve Id bileşenlerinin vektör tabanlı yönetimi |
| **simple_ecs_2** | Geliştirilmiş ECS yapısı, RefCell ve downcast kullanarak dinamik bileşen yönetimi |
| **robotic** | ECS prensipleriyle robot sistemlerinde sensör okuma, pozisyon güncelleme ve kol kontrolü |
| **inside_bevy** | Bevy'nin World, Schedule, Query ve Resource mekanizmalarının temel kullanımı |
| **learning_ecs** | Classic, composition ve bevy_style modüllerini içeren ECS öğrenme projesi |

## Tanım

ECS, karmaşık oyun mantıklarının daha kolay yönetimi için geliştirilmiş mimari bir yaklaşımdır. Eseneklik, modülerlik ve yeniden kullanılabilirlik gibi özellikleri öne çıkarır, **Composition over Inheritance** ilkesini benimser.

**Entity:** Benzersiz ID ile tanımlı basit bir konteynerdır. Gerekli bileşenleri içerir. *(Tower, Player, Enemy, Bullet, Gate birer entity olarak düşünülebilir)*
**Component:** Sadece veri içeren ve Entity nesnelerine eklenen nesnelerdir. Bir entity bir bileşen nesnesinden sadece bir tane içerebilir.
**System:** Belli bileşenlere sahip Entity koleksiyonları üzerinde hareket edebilen, bileşen bazlı entity kümelerini sorgulayabilen fonksiyonlardır.

## ECS Neden Gereklidir?

Bir oyun geliştirirken aktörler, nesneler, bileşenler ve kaynaklar gibi önemli enstrümanlar kullanılır. Bazı durumlarda oyun dünyası içindeki tüm nesnelerin bir hareketi söz konusu iken buna durağan nesneler dahil değildir. Dolayısıyla belli component'lere sahip olan nesneler için gerçekleştirilecek süreçlerde, örneğin sadece hareket etme kabiliyeti olan varlıkların her frame anında bir kurala göre yer değiştirmesi ya da çarpışma ve hasar alma verileri içeren varlıklardan yok olanların sahadan ve oyun nesne koleksiyonlarından çıkartılması gibi devasa süreçlerde veri ile davranışın ayrıştırılması kod yönetimi, kod okunurluğu ve çalışma zamanı performansını artırabilir. Kalıtım bazlı klasik kod pratiklerini içeren oyun sistemlerinde bunu sağlamak çok kolay değildir. ECS burada bir çözüm olarak karşımıza çıkar. Yani nesne sayısının artmasına bağlı olarak oyun motorunun yavaşlaması ve kod ile verinin buna bağlı olarak çok karmaşıklaşması ECS ihtiyacını öne çıkaran konulardır.

ECS'in kazandırdığı bazı avantajlar şöyle sıralanabilir.

- Kod ve veri ayrıldığından veri yeniden yorumlanabilir.
- Kod tek bir Entity yerine birden fazal Entity üzerinde dolaşabilir.
- Sistemler otomatik olarak paralel çalıştırılabilir.
- Sadece belli bileşenleri içeren Entity kümelerinde dolaşmak kolaydır.

_**Unity DOTS** ve **Unreal Mass**'a nazaran Rust için geliştirilmiş olan **Bevy**'nin kullanımı oldukça kolaydır._

## ECS Hakkında Genel Bilgiler

ECS çatısında oyundaki her nesne benzersiz bir tanımlayıcı ile işaretlenir ve bu bir Entity olarak ifade edilir. Entity'lere eklenebilecek verileri içeren **datatype** nesneleri de birer **Component** olarak tasarlanır. Sistemler belli bileşenlere sahip Entity kümelerinin dolaşılması için kullanılır.

> ECS, kodun yeniden kullanılabilirliğini _(Reusability)_ artırır ve bunu yaparken veriyi davranışlardan _(Behavior)_ ayırır.

Örneğin çok basit anlamda **Tower Defence** benzeri bir oyunu düşünelim. **Entity** ve **Component** ilişkilerini aşağıdaki gibi özetleyebiliriz.

```text
+----------------+----------+----------+----------+----------+
|   Components   | Player   |  Tower   |  Enemy   | Bullet   |
+----------------+----------+----------+----------+----------+
| Position       | (x,y)    | (x, y)   | (x, y)   | (x, y)   |
| Health         | (hp)     |          | (hp)     |          |
| Damage         | (dmg)    | (dmg)    | (dmg)    |          |
| Range          |          | (range)  |          |          |
| Velocity       |          | (vx, vy) | (vx, vy) |          |
| Inventory      | (inv)    |          |          |          |
+----------------+----------+----------+----------+----------+
```

## Composition Over Inheritance

Entity Component System, kalıtım *(Inheritance)* yerine **Composition over Inheritance** yaklaşımını kullanır. Bir Entity tür hiyerarşisi yerine onunla ilişkili bileşenler _(Component)_ tarafından tanımlanır. Sistemler, istenen bileşenlere sahip Entity koleksiyonları üzerinde harket ederek çeşitli işlemler icra edebilir. Her iki uygulama arasındaki farkı yorumlamak için **classic** ve **composition** isimli Rust projelerinin kodlarına bakılabilir.

> Projeleri çalıştırmak için kodlar indirildikten sonra komut satırından aşağıdaki gibi ilerlenebilir. Tüm projeler aynı workspace altında birleştirilmiştir. Root klasörde aşağıdaki şekilde istenen uygulamalar çalıştırılabilir.

```shell
cargo run --bin classic

cargo run --bin composition

cargo run --bin hello_ecs

cargo run --bin simple_ecs

cargo run --bin simple-ecs_2

cargo run --bin robotic

cargo run --bin game_without_bevy

cargo run --bin game_with_bevy
```

## ECS ile OOP Arasındaki Farklar

- OOP tarafından kalıtım _(Inheritance)_ birinci sınıf vatandaş _(Citizen)_ iken ECS çatısında bu Composition'dır.
- OOP veriyi encapsulate etmeyi önerirken ECS, Plain Old Data nesnelerini kullanmayı teşvik eder.
- ECS veriyi davranışlardan ayırırken, OOP verileri davranışla birleştiren bir yol önerir.

## Tarihçe

- Kayıtlara göre ECS'in ilk öncüsü 1998 yılında yayınlanan **Thief: The Dark Project** isimli oyundur. Bu oyunda kullanılan ECS motoru sonrasında devamında ve **System Shock 2** oyununda kullanılmış.
- **2007** yılında ECS sistemlerinin **MMOG** türünde kullanımı ile ilgili **Adam Martin** tarafından [detaylı bir yazı](https://t-machine.org/index.php/2007/09/03/entity-systems-are-the-future-of-mmog-development-part-1/) yayınlandı.
- 2015 yılında Apple, ECS'in bir uyarlamasını içeren ve iOS, macOS ve tvOS'larda oyun geliştirmek için kullanılan **GameplayKit** isimli bir **framework** yayınladı.
- 2018 yılında **Sander Mertens** [flecs](https://github.com/SanderMertens/flecs) isimli bir ECS Framework oluşturdu. Bu framework C ve C++ için yapılmış bir uyarlamadır.
- 2018 yılında ise Unity platformu da ECS'i kullanan bir demo yayınladı.

> Güncelleme: Unity tarafında DOTS-Data Oriented Technology Stack isimli bir çatı tanıtıldı. [](https://unity.com/dots) Bu çatı içerisinde ECS, C# Job System ve Burst Compiler gibi enstrümanlar yer alıyor.

## ECS'in Kullanıldığı Diğer Alanlar

ECS, oyun geliştirme dışında da çeşitli alanlarda kullanılabilir. İşte bazı örnekler:

- **Simülasyon Yazılımları :** ECS, karmaşık sistemlerin modellenmesi gereken simülasyon yazılımlarında kullanılabilir. Örneğin, trafik simülasyonlarını ele alalım. Arabalar ve yayalar birer Entity olarak düşünülebilir. Araçların konumları, hızları ve yönleri birer bileşen *(Component)* olarak tasarlanabilir. Sistemler, çarpışma algılama ve rota planlama gibi işlevleri yürütebilir.
- **Robotik/IoT :** Robitik veya IoT sistemlerde bir cihazın parçalarını ve etkileşimlerini yönetmek için ECS'den yararlanılabilir. Örneğin bir robotun farklı uzuvları birer Entity olarak düşünülebilir. Kolları, sensörleri, ayakları vs. Yine bu nesnelerin konumları, anlık durumları birer bileşen olarak düşünülebilir. Sistemler bu parçaların koordinasyon ve kontrolünü yönetir ve gezinme, rota belirleme, metrik ölçümleyip durum tespiti yapma, çevre tarama ve basit görevleri etkinleştirir.
- **Data-Driven Mimariler :** Büyük verilerin *(Big Data)* işlenmesi ve analizinde kullanılabilir. Veri akışları *(Data Streams)* birer Entity olabilir, metadata ve transformation kuralları ise birer bileşen olarak düşünülebilir. Sistemler verileri bu kurallara göre işler ve analiz eder.
- **Sanal/Artırılmış Gerçeklik (VR/AR) :** Sanal ortamdaki nesneler birer Entity olarak temsil edebilir. Bu nesnelerin fiziksel özellikleri ve davranışları ise birer bileşen olarak düşünülebilir. Sistemler rendering, etkileşim ve gerçek hayat fizik ilkelerini işleyebilir.
- **UI Frameworks :** Bu tip bir framework içerisinde Button, Slider, CheckBox, TextBox gibi unsular birer Entity olarak düşünüldüğünde boyutları, renkleri, durumları vb unsurlar da bileşen olarak tesis edilebilir. Sistemler çeşitli bileşenlere sahip entity nesnelerinin render edilmesi veya kullanıcı ile etkileşimini yönetebilir.

## Bevy ECS Hakkında

Bevy, ECS çatısının uygulanabildiği en ergonomik çatılardan birisidir. Bileşenler *(Components)* struct olarak tanımlanırken, sistemler birer fonksiyon olarak yazılır. Bevy ECS, oyun dünyası *(World)* , planlayıcı *(Scheduler)* , komut listesi *(Command List)*, kaynaklar *(Resources)* , sistem setleri *(System Sets)* ve bundle gibi enstrümanları da sağlayarak programcının işini epeyce kolaylaştırır.

- **World:** ECS içinde kullanılanacak tüm veri ve kaynakları içeren nesnedir. Entity'ler ve bileşenlerini, kaynakları ve sistemler arası mesajlaşmalar için de kullanılabilecek Event'leri içerir.
- **Resources:** World içerisindeki global değişkenler olarak düşünülebilir. *(Elapsed Times: örneğin belli aralıklarda sahaya bir göktaşının inmesi, Assets: her türlü ses ve grafik, Renderers)*
- **Schedule:** Sistemlerin belli sırada çalıştırılmasını sağlamak için kullanılabilen enstrüman.
- **Commands:** World nesnesi içerisinde yapısal değişiklikler için kullanılır. Örneğin Entity'lerin spawn/despawn edilmeleri, Entity nesnelerine Component'lerin eklenmesi, Resource nesnelerinin yönetimi.
- **System Sets:** Bazı özelliklerin birden fazla sisteme kolayca uygulanabilmesi için kullanılabilen enstrümandır.

## Kaynaklar

- Kendi ECS çatımızı yazmak istersek Ian'ın [şu adresteki](https://ianjk.com/ecs-in-rust/) öğretisine bakabiliriz. simple_ecs ve simple_ecs_2'de bu öğretinin pratik uygulaması yer almaktadır.
- [Entity Component System - Wikipedia](https://en.wikipedia.org/wiki/Entity_component_system)
- [Rust Entity Component Systems: ECS Libraries for Rust Game Dev 🧩 | Rodney Lab](https://rodneylab.com/rust-entity-component-systems/)
- [Bevy Engine](https://bevyengine.org/)
- [Build Your First Game in Bevy and Rust - Step by Step Tutorial](https://www.youtube.com/watch?v=E9SzRc9HkOg)
- [ECS with Bevy Game Engine](https://www.youtube.com/watch?v=iH5NkbaXi0o)
- [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/introduction.html)
