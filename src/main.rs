use std::collections::HashMap;
use std::io;

fn main() {
    // Menampilkan menu
    println!("Pilih menu:");
    println!("1. Tambah barang");
    println!("2. Lihat barang");
    println!("3. Edit barang");
    println!("4. Hapus barang");

    // Meminta input pengguna
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut pendataan_barang = HashMap::new();

    // Memproses input pengguna
    match input {
        "1" => {
            // Menambahkan barang baru
            println!("Tambahkan barang:");
            let mut task = String::new();
            io::stdin().read_line(&mut task).unwrap();
            let mut description = String::new();
            io::stdin().read_line(&mut description).unwrap();
            pendataan_barang.insert(task.trim(), description.trim());
        }
        "2" => {
            // Menampilkan daftar barang
            println!("Daftar barang:");
            for (task, description) in pendataan_barang.iter() {
                println!("* {}: {}", task, description);
            }
        }
        "3" => {
            // Mengedit barang
            println!("Edit barang:");
            println!("Masukkan nama barang yang ingin diedit:");
            let mut task = String::new();
            io::stdin().read_line(&mut task).unwrap();
            match pendataan_barang.get_mut(task.trim()) {
                Some(description) => {
                    // Edit deskripsi barang
                    println!("Masukkan deskripsi barang yang baru:");
                    let mut new_description = String::new();
                    io::stdin().read_line(&mut new_description).unwrap();
                    *description = new_description.trim();
                }
                None => {
                    // Barang tidak ditemukan
                    println!("Barang tidak ditemukan");
                }
            }
        }
        "4" => {
            // Menghapus barang
            println!("Hapus barang:");
            println!("Masukkan nama barang yang ingin dihapus:");
            let mut task = String::new();
            io::stdin().read_line(&mut task).unwrap();
            pendataan_barang.remove(task.trim());
        }
        _ => {
            // Input tidak valid
            println!("Input tidak valid");
        }
    }
}
