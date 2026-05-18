# Asynchronous Programming - Kevin Cornellius Widjaja (2406428781)

## Experiment 1.2: Understanding how it works

![Screenshot Eksperimen 1.2](docs/images/1_2.png)

**Penjelasan:**
Saya menambahkan `println!("Kevin's Komputer: hey hey");` tepat setelah `spawner.spawn(...)` namun sebelum `drop(spawner);`. Ketika program dijalankan, output yang saya dapat adalah "Kevin's Komputer: hey hey" tercetak lebih dulu sebelum "howdy!" dan "done!".

Ini terjadi karena `spawner.spawn()` tidak langsung mengeksekusi future secara synchronous. Yang terjadi adalah future tersebut hanya dimasukan ke dalam sebuah task queue (antrian). Setelah itu, program utama (main thread) melanjutkan eksekusi ke baris kode berikutnya secara synchronous - yaitu perintah println "hey hey". Baru ketika `executor.run()` dipanggil di akhir, semua task yang ada di dalam queue akan diproses secara asynchronous oleh executor.

Intinya: spawn itu hanya untuk menyimpan task ke queue, sedangkan eksekusi yang sesungguhnya baru terjadi ketika executor menjalankan task tersebut.