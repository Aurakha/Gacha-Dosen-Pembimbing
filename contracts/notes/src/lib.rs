#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk profil Dosen
#[contracttype]
#[derive(Clone, Debug)]
pub struct Dosen {
    name: String,
    expertise: String, // Contoh: "Spesialis Laravel & Tailwind" atau "Ahli UI/UX Figma"
}

// Struktur data untuk menyimpan riwayat hasil gacha
#[contracttype]
#[derive(Clone, Debug)]
pub struct GachaResult {
    student_name: String,
    dosen: Dosen,
}

// Storage keys untuk memisahkan tempat penyimpanan di blockchain
const DOSEN_POOL: Symbol = symbol_short!("DOSEN_PL");
const GACHA_HISTORY: Symbol = symbol_short!("HISTORY");

#[contract]
pub struct GachaDosenContract;

#[contractimpl]
impl GachaDosenContract {
    
    // 1. Fungsi untuk menambahkan dosen ke dalam "Pool Gacha"
    pub fn add_dosen(env: Env, name: String, expertise: String) -> String {
        let mut dosen_list: Vec<Dosen> = env.storage().instance().get(&DOSEN_POOL).unwrap_or(Vec::new(&env));
        
        let dosen_baru = Dosen {
            name,
            expertise,
        };
        
        dosen_list.push_back(dosen_baru);
        env.storage().instance().set(&DOSEN_POOL, &dosen_list);
        
        return String::from_str(&env, "Dosen berhasil masuk ke pool gacha!");
    }

    // 2. Fungsi untuk melihat daftar semua dosen yang ada di pool
    pub fn get_dosen_pool(env: Env) -> Vec<Dosen> {
        return env.storage().instance().get(&DOSEN_POOL).unwrap_or(Vec::new(&env));
    }

    // 3. Inti Aplikasi: Fitur Gacha (Roll)
    pub fn roll_gacha(env: Env, student_name: String) -> GachaResult {
        // Ambil daftar dosen yang tersedia
        let dosen_list: Vec<Dosen> = env.storage().instance().get(&DOSEN_POOL).unwrap_or(Vec::new(&env));
        
        // Cek jika belum ada dosen di dalam pool untuk mencegah error
        if dosen_list.is_empty() {
            panic!("Pool dosen kosong! Isi pool terlebih dahulu.");
        }

        // Tentukan total dosen yang ada dan ubah tipe datanya ke u64
        let total_dosen = dosen_list.len() as u64;
        
        // Tarik angka acak dan berikan anotasi tipe u64 secara eksplisit
        let random_index_u64: u64 = env.prng().gen_range(0..total_dosen);
        
        // Kembalikan ke u32 karena fungsi .get() di Soroban membutuhkan u32
        let random_index = random_index_u64 as u32;
        
        // Ambil data dosen berdasarkan index yang terpilih secara acak
        let dosen_terpilih = dosen_list.get(random_index).unwrap();

        // Bungkus hasilnya
        let result = GachaResult {
            student_name,
            dosen: dosen_terpilih,
        };

        // Simpan hasil ini ke riwayat (history) blockchain
        let mut history: Vec<GachaResult> = env.storage().instance().get(&GACHA_HISTORY).unwrap_or(Vec::new(&env));
        history.push_back(result.clone());
        env.storage().instance().set(&GACHA_HISTORY, &history);

        // Kembalikan data hasil gacha
        return result;
    }

    // 4. Melihat riwayat siapa dapat dosen siapa
    pub fn get_history(env: Env) -> Vec<GachaResult> {
        return env.storage().instance().get(&GACHA_HISTORY).unwrap_or(Vec::new(&env));
    }
}

mod test;