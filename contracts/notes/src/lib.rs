#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Report {
    id: u64,
    title: String,
    description: String,
    category: String,
    status: String,
}

const REPORT_DATA: Symbol = symbol_short!("REPORTS");

#[contract]
pub struct CampusBoardContract;

#[contractimpl]
impl CampusBoardContract {

    // Ambil semua laporan
    pub fn get_reports(env: Env) -> Vec<Report> {
        env.storage()
            .instance()
            .get(&REPORT_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah laporan baru
    pub fn create_report(
        env: Env,
        title: String,
        description: String,
        category: String,
    ) -> String {

        let mut reports: Vec<Report> = env.storage()
            .instance()
            .get(&REPORT_DATA)
            .unwrap_or(Vec::new(&env));

        let report = Report {
            id: env.prng().gen::<u64>(),
            title,
            description,
            category,
            status: String::from_str(&env, "OPEN"),
        };

        reports.push_back(report);

        env.storage().instance().set(&REPORT_DATA, &reports);

        String::from_str(&env, "Report berhasil dibuat")
    }

    // Update status laporan
    pub fn update_status(
        env: Env,
        id: u64,
        new_status: String,
    ) -> String {

        let mut reports: Vec<Report> = env.storage()
            .instance()
            .get(&REPORT_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..reports.len() {
            let mut report = reports.get(i).unwrap();

            if report.id == id {
                report.status = new_status;

                reports.set(i, report);

                env.storage().instance().set(&REPORT_DATA, &reports);

                return String::from_str(&env, "Status berhasil diupdate");
            }
        }

        String::from_str(&env, "Report tidak ditemukan")
    }

    // Hapus laporan
    pub fn delete_report(env: Env, id: u64) -> String {

        let mut reports: Vec<Report> = env.storage()
            .instance()
            .get(&REPORT_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..reports.len() {
            if reports.get(i).unwrap().id == id {

                reports.remove(i);

                env.storage().instance().set(&REPORT_DATA, &reports);

                return String::from_str(&env, "Report berhasil dihapus");
            }
        }

        String::from_str(&env, "Report tidak ditemukan")
    }
}