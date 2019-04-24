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
```rust
