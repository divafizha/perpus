use std::io;
use std::collections::HashMap;

// Mendefinisikan struktur untuk merepresentasikan seorang pengunjung
struct LibraryVisitor {
    id: u32,
    nama: String,
    email: String,
    alamat: String,
}

// Mendefinisikan struktur untuk merepresentasikan perpustakaan dengan kumpulan pengunjung
struct Library {
    visitors: HashMap<u32, LibraryVisitor>,
    visitor_stack: Stack<u32>,
    visitor_queue: Queue<u32>,
}

// Mendefinisikan struktur untuk merepresentasikan Stack
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

// Mendefinisikan struktur untuk merepresentasikan Queue
struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue { items: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// Implementasi metode-metode untuk struktur Library
impl Library {
    // Konstruktor untuk membuat objek Library baru
    fn new() -> Library {
        Library {
            visitors: HashMap::new(),
            visitor_stack: Stack::new(),
            visitor_queue: Queue::new(),
        }
    }

    // Metode untuk menambahkan pengunjung baru ke perpustakaan
    fn add_visitor(&mut self, id: u32, nama: String, email: String, alamat: String) {
        self.visitors.insert(
            id,
            LibraryVisitor {
                id,
                nama,
                email,
                alamat,
            },
        );
    }

    // Metode untuk melihat informasi tentang seorang pengunjung berdasarkan ID
    fn view_visitor(&self, id: u32) {
        match self.visitors.get(&id) {
            Some(visitor) => println!("ID: {}, Nama: {}, Email: {}, Alamat: {}", visitor.id, visitor.nama, visitor.email, visitor.alamat),
            None => println!("No visitor found with id: {}", id),
        }
    }

    // Metode untuk mengedit informasi tentang seorang pengunjung berdasarkan ID
    fn edit_visitor(&mut self, id: u32, nama: String, email: String, alamat: String) {
        match self.visitors.get_mut(&id) {
            Some(visitor) => {
                visitor.nama = nama;
                visitor.email = email;
                visitor.alamat = alamat;
                println!("Visitor with id: {} updated successfully", id);
            }
            None => println!("No visitor found with id: {}", id),
        }
    }

    // Metode untuk menghapus seorang pengunjung berdasarkan ID
    fn remove_visitor(&mut self, id: u32) {
        match self.visitors.remove(&id) {
            Some(_) => println!("Visitor with id: {} removed successfully", id),
            None => println!("No visitor found with id: {}", id),
        }
    }

    // Metode untuk menambahkan pengunjung ke dalam Stack
    fn push_visitor_to_stack(&mut self, id: u32) {
        self.visitor_stack.push(id);
    }

    // Metode untuk menampilkan pengunjung dari Stack
    fn pop_visitor_from_stack(&mut self) {
        if let Some(id) = self.visitor_stack.pop() {
            self.view_visitor(id);
        } else {
            println!("Stack is empty");
        }
    }

    // Metode untuk menambahkan pengunjung ke dalam Queue
    fn enqueue_visitor(&mut self, id: u32) {
        self.visitor_queue.enqueue(id);
    }

    // Metode untuk menampilkan pengunjung dari Queue
    fn dequeue_visitor(&mut self) {
        if let Some(id) = self.visitor_queue.dequeue() {
            self.view_visitor(id);
        } else {
            println!("Queue is empty");
        }
    }
}

// Fungsi utama untuk menjalankan program
fn main() {
    // Membuat objek Library baru
    let mut library = Library::new();

    // Loop utama program
    loop {
        // Menampilkan opsi menu
        println!("Menu: \n1. Add Visitor\n2. View Visitor\n3. Edit Visitor\n4. Remove Visitor\n5. Push to Stack\n6. Pop from Stack\n7. Enqueue to Queue\n8. Dequeue from Queue\n9. Exit");
        // Membaca pilihan pengguna dari input
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Cocokkan pilihan pengguna untuk melakukan tindakan yang sesuai
        match choice {
            1 => {
                // Menambahkan pengunjung baru
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let mut nama = String::new();
                io::stdin().read_line(&mut nama).expect("Failed to read line");
                let mut email = String::new();
                io::stdin().read_line(&mut email).expect("Failed to read line");
                let mut alamat = String::new();
                io::stdin().read_line(&mut alamat).expect("Failed to read line");
                library.add_visitor(id, nama.trim().to_string(), email.trim().to_string(), alamat.trim().to_string());
            }
            2 => {
                // Melihat informasi tentang seorang pengunjung
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                library.view_visitor(id);
            }
            3 => {
                // Mengedit informasi tentang seorang pengunjung
                let mut id

 = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let mut nama = String::new();
                io::stdin().read_line(&mut nama).expect("Failed to read line");
                let mut email = String::new();
                io::stdin().read_line(&mut email).expect("Failed to read line");
                let mut alamat = String::new();
                io::stdin().read_line(&mut alamat).expect("Failed to read line");
                library.edit_visitor(id, nama.trim().to_string(), email.trim().to_string(), alamat.trim().to_string())
            }
            4 => {
                // Menghapus seorang pengunjung
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                library.remove_visitor(id);
            }
            5 => {
                // Push visitor ke dalam Stack
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                library.push_visitor_to_stack(id);
            }
            6 => {
                // Pop visitor dari Stack
                library.pop_visitor_from_stack();
            }
            7 => {
                // Enqueue visitor ke dalam Queue
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                library.enqueue_visitor(id);
            }
            8 => {
                // Dequeue visitor dari Queue
                library.dequeue_visitor();
            }
            9 => {
                // Keluar dari program
                println!("Exiting program");
                break;
            }
            _ => {
                // Tangani pilihan yang tidak valid
                println!("Invalid choice. Please choose a number between 1 and 9.");
            }
        }
    }
}