#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, symbol_short};

// Struct ini merepresentasikan baris data spesies yang sama persis dengan versi Web2,
// namun sekarang disimpan on-chain di blockchain Stellar.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BiodiversityRecord {
    pub id: u32,
    pub nama_ilmiah: String,   // Contoh: Panthera tigris sondaica
    pub nama_lokal: String,    // Contoh: Harimau Sumatra
    pub status_iucn: String,   // Contoh: Critically Endangered (Kritis)
    pub lokasi_ditemukan: String, // Contoh: Taman Nasional Way Kambas
}

const DATABASE_KEY: Symbol = symbol_short!("BIO_DB");

#[contract]
pub struct BiodiversityDatabaseContract;

#[contractimpl]
impl BiodiversityDatabaseContract {

    // 1. CREATE (Tambah Data Spesies Baru)
    pub fn add_record(env: Env, id: u32, nama_ilmiah: String, nama_lokal: String, status_iucn: String, lokasi_ditemukan: String) {
        let mut db: soroban_sdk::Vec<BiodiversityRecord> = env
            .storage()
            .instance()
            .get(&DATABASE_KEY)
            .unwrap_or_else(|| soroban_sdk::Vec::new(&env));

        // Pastikan ID belum digunakan untuk mencegah duplikasi data
        for i in 0..db.len() {
            let record = db.get(i).unwrap();
            if record.id == id {
                panic!("ID Spesies sudah terdaftar di blockchain!");
            }
        }

        let new_record = BiodiversityRecord {
            id,
            nama_ilmiah,
            nama_lokal,
            status_iucn,
            lokasi_ditemukan,
        };

        db.push_back(new_record);
        env.storage().instance().set(&DATABASE_KEY, &db);
    }

    // 2. READ (Ambil/Tampilkan Semua Data Spesies untuk Web)
    pub fn get_all_records(env: Env) -> soroban_sdk::Vec<BiodiversityRecord> {
        env.storage()
            .instance()
            .get(&DATABASE_KEY)
            .unwrap_or_else(|| soroban_sdk::Vec::new(&env))
    }

    // 3. UPDATE (Perbarui Data Jika Ada Perubahan Status Konservasi/Lokasi)
    pub fn update_record(env: Env, id: u32, new_status: String, new_lokasi: String) -> bool {
        let mut db: soroban_sdk::Vec<BiodiversityRecord> = env
            .storage()
            .instance()
            .get(&DATABASE_KEY)
            .unwrap_or_else(|| soroban_sdk::Vec::new(&env));

        let mut updated = false;

        for i in 0..db.len() {
            let mut record = db.get(i).unwrap();
            if record.id == id {
                record.status_iucn = new_status.clone();
                record.lokasi_ditemukan = new_lokasi.clone();
                db.set(i, record);
                updated = true;
                break;
            }
        }

        if updated {
            env.storage().instance().set(&DATABASE_KEY, &db);
        }
        updated
    }

    // 4. DELETE (Hapus Data Jika Terjadi Kesalahan Input Data)
    pub fn delete_record(env: Env, id: u32) -> bool {
        let mut db: soroban_sdk::Vec<BiodiversityRecord> = env
            .storage()
            .instance()
            .get(&DATABASE_KEY)
            .unwrap_or_else(|| soroban_sdk::Vec::new(&env));

        let mut index_to_remove: Option<u32> = None;

        for i in 0..db.len() {
            let record = db.get(i).unwrap();
            if record.id == id {
                index_to_remove = Some(i);
                break;
            }
        }

        match index_to_remove {
            Some(index) => {
                db.remove(index);
                env.storage().instance().set(&DATABASE_KEY, &db);
                true
            },
            None => false,
        }
    }
}