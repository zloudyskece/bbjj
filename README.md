# Sistem  Pengukuran dan Kontrol Level Tangki Industri dengan Valve Berbasis Rust

Project ini adalah aplikasi berbasis CLI (Command Line Interface) yang dirancang untuk mensimulasikan sistem pemantauan level air pada tangki industri secara *real-time*. Program ini menggunakan konsep **Pemrograman Berbasis Objek (Struct & Implementation)** dalam bahasa Rust untuk memberikan akurasi data dan logika otomasi sistem.

---

## Fitur Utama

-   **Kalkulasi Volume Akurat**: Mengonversi pembacaan sensor (meter) ke dalam satuan liter secara otomatis.
-   **Sistem Alarm Otomatis**: Deteksi dini kondisi *Low Low (LL)* dan *High High (HH)* untuk mencegah kekosongan atau luapan.
-   **Kontrol Valve & Pompa**: Simulasi logika otomasi untuk membuka/menutup Inlet Valve, Outlet Valve, dan mengaktifkan Pompa berdasarkan level air.
-   **Dashboard Visual**: Tampilan dashboard yang rapi dengan indikator persentase dan status sistem.

---

## Logika Sistem

Program ini bekerja berdasarkan ambang batas (threshold) yang ditentukan pengguna:

| Kondisi | Status Sensor | Inlet Valve | Pump | Outlet Valve | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Batas Bawah** | $\le$ Low Low (LL) | **OPEN** | **ON** | CLOSE | ⛔ ALARM (Kritis) |
| **Normal** | Antara LL & HH | CLOSE | OFF | CLOSE | ✅ NORMAL |
| **Batas Atas** | $\ge$ High High (HH) | CLOSE | OFF | **OPEN** | ⛔ ALARM (Meluap) |

---

## Cara Menjalankan Program

### Prasyarat
Pastikan kamu sudah menginstal **Rust** di komputermu. Jika belum, instal melalui [rustup.rs](https://rustup.rs/).

### Langkah-langkah

1.  **Clone atau Simpan File**
    Simpan kode sumber sebagai `main.rs`.

2.  **Kompilasi Program**
    Buka terminal dan jalankan perintah berikut:
    ```bash
    rustc main.rs
    ```

3.  **Jalankan Program**
    Setelah kompilasi selesai, jalankan file executable-nya:
    - **Windows:**
      ```bash
      ./main.exe
      ```
    - **Linux/macOS:**
      ```bash
      ./main
      ```

---

## Contoh Penggunaan

![App Screenshot](/foto%20input.jpeg)


**Hasil Output:**



![App Screenshot](/foto%20output.jpeg)

