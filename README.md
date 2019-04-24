# assignment2_3145152099

## Membuat Penjelasan Singkat Penyelesaian Exercism Level Medium

---

### Clock - Medium
Level medium pertama yang saya bisa selesaikan adalah Clock.
Cara saya untuk menyelesaikan adalah sebagai berikut:
1. Melihat intruksi terlebih dahulu agar paham apa yang harus dikerjakan.
2. Memeriksa 'clock.rs' yaitu file test untuk memastikan apa output yang diinginkan
3. Memulai mencari dokumen rust yang memungkinkan membantu pengerjaan
  
Yang dibutuhkan pertama kali untuk menghilangkan error adalah `crate`, `derive`, `impl`
```rust
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:02}:{:02}", &self.hours, &self.minutes)
  }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}:{}", self.hours, self.minutes)
    }
}
```
```rust
extern crate clock;
```
```rust
use std::fmt;
#[derive(PartialEq)]
```
Kemudian mulai perhatikan function yang harus diselesaikan
```rust
 pub fn add_minutes(self, minutes: i32) -> Self {
    let mut h_add = minutes / 60;
    let m_add = minutes % 60;

    if m_add < 60 && (self.minutes + m_add) >= 60 && m_add > 0 {
      h_add += 1;
    } else if m_add < 0 && (self.minutes + m_add) < 0 {
      h_add -= 1;


```
function diatas merupakan function untuk menambahkan menit dan kemudian jam.
jika lebih dari atau kurang dari 60 (menit) makan akan menambahkan dan mengurai sisa menit

```rust
    Clock::new(
      all(self.hours + h_add, 24),
      all(self.minutes + m_add, 60),
    )
 ```
 mendeclare bahwa jam dalam sehari ada 24 dan menit dalam jam ada 60
 
 ```rust
 fn all(value: i32, around: i32) -> i32 {
  let mut all_value = value;
  if all_value < 0 {
    while all_value < 0 {
      all_value += around;
    }
  } else {
    all_value %= around;
  }
  all_value
}
```
secara keseluruhan dalam clock

