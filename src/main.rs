use std::io;

#[derive(Debug)]
struct Data {
    id: i32,
    name: String,
}

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, data: T) {
        self.data.push(data);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn enqueue(&mut self, data: T) {
        self.data.push(data);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.data.pop()
    }
}

fn main() {
    // Buat struktur data untuk menyimpan data
    let mut data = Vec::new();

    // Buat stack dan queue
    let mut stack: Stack<Data> = Stack::new();
    let mut queue: Queue<Data> = Queue::new(); // Antrian untuk menyimpan elemen bertipe Data


    // Menu
    loop {
        println!("Menu:");
        println!("1. Tambah data");
        println!("2. Tampilkan data");
        println!("3. Edit data");
        println!("4. Hapus data");
        println!("5. Stack");
        println!("6. Queue");
        println!("7. Keluar");

        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).unwrap();
        pilihan = pilihan.trim().to_string();

        match pilihan.as_str() {
            "1" => {
                // Tambah data
                let data1 = Data {
                    id: 1,
                    name: "Book".to_string(),
                };
                data.push(data1);
            }
            "2" => {
                // Tampilkan data
                for d in data.iter() {
                    println!("ID: {} | Nama: {}", d.id, d.name);
                }
            }
            "3" => {
                // Edit data
                let _index = 0;
                println!("Masukkan indeks data yang ingin diedit: ");
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                let _index: usize = index.trim().parse().unwrap();

                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                index = index.trim().to_string();  // Pindahkan trim()
                let index: usize = index.parse().unwrap();

                let data_edit = Data {
                    id: data[index].id,
                    name: "Bard".to_string(),
                };
                data[index] = data_edit;
            }
            "4" => {
                // Hapus data
                let _index = 0;
                println!("Masukkan indeks data yang ingin dihapus: ");
                let mut index = String::new();     // Deklarasikan index sebagai String
                io::stdin().read_line(&mut index).unwrap();
                let _index: usize = index.trim().parse().unwrap();  // Parse menjadi usize

                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                index = index.trim().to_string();  // Pindahkan trim() ke sini
                let index: usize = index.parse().unwrap();

                data.remove(index);
            }
            "5" => {
                // Stack
                println!("Stack");
                while let Some(_data) = stack.pop() {
                    impl std::fmt::Display for Data {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "Data(id: {}, name: {})", self.id, self.name)
                        }
                    }
                }
            }
            "6" => {
                // Queue
                println!("Queue");
                while let Some(data) = queue.dequeue() {
                    println!("Data: {}", data);
                }
            }
            "7" => {
                // Keluar
                break;
            }
            _ => {
                // Pilihan tidak valid
                println!("Pilihan tidak valid!");
            }
        }
    }
}
