#![allow(dead_code, unused_variables, unused_imports)]
/*
Этот файл содержит код для определения моделей или структур базы данных. 
Он определяет структуру таблиц или коллекций в вашей базе данных и соответствующие поля.
*/
use std::collections::HashMap;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use chrono::Utc;
use diesel::{ Insertable, Queryable, Selectable, Table };
use serde::{ Deserialize, Serialize };
use crate::database::schema::animals;
use diesel::SelectableHelper;
#[derive(Serialize, Deserialize, Debug, Selectable, Insertable, Queryable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = animals)]
pub struct AnimalData {
    pub id: i32, // id
    pub o: Option<String>, // Отец
    pub m: Option<String>, // Мать
    pub oo: Option<String>, // Отец отца
    pub mo: Option<String>, // Мать отца
    pub om: Option<String>, // Отец матери
    pub mm: Option<String>, // Мать матери
    pub flock_name: Option<String>, // Flock Name
    pub herd_book_section: Option<String>, // Herd Book Section
    pub inbreeding: Option<String>, // Inbreeding
    pub keeper: Option<String>, // Keeper
    pub owner: Option<String>, // Owner
    pub weaned_progeny: Option<String>, // Weaned Progeny
    pub flocks_with_weaned_progeny: Option<String>, // Flocks with Weaned Progeny
    pub daughters_with_weaned_progeny: Option<String>, // Daughters with Weaned Progeny
    pub flocks_with_daughters_with_weaned_progeny: Option<String>, // Flocks with Daughters with Weaned Progeny
    pub pre_wean_direct: Option<String>, // Pre-Wean Direct
    pub pre_wean_maternal: Option<String>, // Pre-Wean Maternal
    pub wean_direct: Option<String>, // Wean Direct
    pub wean_maternal: Option<String>, // Wean Maternal
    pub pre_wean_combined: Option<String>, // Pre-Wean Combined
    pub wean_combined: Option<String>, // Wean Combined
    pub post_wean_direct: Option<String>, // Post-Wean Direct
    pub number_of_lambs_weaned: Option<String>, // Number of Lambs Weaned
    pub total_weight_weaned: Option<String>, // Total Weight Weaned
    pub age_at_first_lambing: Option<String>, // Age at First Lambing
    pub inter_lamb_period: Option<String>, // Inter-Lamb Period
    pub growth_index: Option<String>, // Growth Index
    pub reproduction_index: Option<String>, // Reproduction Index
    pub logix_merit_index: Option<String>, // Logix Merit Index
    pub dorper_logix_merit_index: Option<String>, // Dorper Logix Merit Index
    pub url_animal_card: Option<String>, // адрес карточки животного
    pub breed: String, // порода
    pub parse_card: i32, // Признак добавления данных из карточки 0 - нет, 1 - да
    pub fdm: Option<String>, // Flock Name
    pub name: Option<String>, // Identification Number
    pub comp_no: Option<String>, // S.A. Reg Number
    pub id_animal: Option<String>, // идентификатор животного, unique=True
    pub byear: Option<String>, // Birth Date
    pub inbred: Option<String>, // Inbreeding
    pub status: Option<String>, // статус животного
    pub sex: Option<String>, // пол
    pub lmi_1: Option<String>, // LMI
    pub lmi_2: Option<String>, // LMI
    pub lmi_3: Option<String>, // LMI
    pub lmi_4: Option<String>, // LMI
    pub growth_ind_1: Option<String>, // Growth Ind
    pub growth_ind_2: Option<String>, // Growth Ind
    pub growth_ind_3: Option<String>, // Growth Ind
    pub growth_ind_4: Option<String>, // Growth Ind
    pub reprod_ind_1: Option<String>, // Reprod. Ind
    pub reprod_ind_2: Option<String>, // Reprod. Ind
    pub reprod_ind_3: Option<String>, // Reprod. Ind
    pub reprod_ind_4: Option<String>, // Reprod. Ind
    pub wean_dir_1: Option<String>, // Wean Dir
    pub wean_dir_2: Option<String>, // Wean Dir
    pub wean_dir_3: Option<String>, // Wean Dir
    pub wean_dir_4: Option<String>, // Wean Dir
    pub wean_mat_1: Option<String>, // Wean Mat
    pub wean_mat_2: Option<String>, // Wean Mat
    pub wean_mat_3: Option<String>, // Wean Mat
    pub wean_mat_4: Option<String>, // Wean Mat
    pub wean_comb_1: Option<String>, // Wean Comb
    pub wean_comb_2: Option<String>, // Wean Comb
    pub wean_comb_3: Option<String>, // Wean Comb
    pub wean_comb_4: Option<String>, // Wean Comb
    pub n_lambs_weaned_1: Option<String>, // N Lambs Weaned
    pub n_lambs_weaned_2: Option<String>, // N Lambs Weaned
    pub n_lambs_weaned_3: Option<String>, // N Lambs Weaned
    pub n_lambs_weaned_4: Option<String>, // N Lambs Weaned
    pub tww_1: Option<String>, // TWW
    pub tww_2: Option<String>, // TWW
    pub tww_3: Option<String>, // TWW
    pub tww_4: Option<String>, // TWW
    pub afl_1: Option<String>, // AFL
    pub afl_2: Option<String>, // AFL
    pub afl_3: Option<String>, // AFL
    pub afl_4: Option<String>, // AFL
    pub ilp_1: Option<String>, // ILP
    pub ilp_2: Option<String>, // ILP
    pub ilp_3: Option<String>, // ILP
    pub ilp_4: Option<String>, // ILP
    pub create_at: String, // Create At
    pub update_at: String, // Update At
}

#[derive(Serialize, Insertable)]
#[diesel(table_name = animals)]
pub struct NewAnimal {
    pub o: Option<String>, // Отец
    pub m: Option<String>, // Мать
    pub oo: Option<String>, // Отец отца
    pub mo: Option<String>, // Мать отца
    pub om: Option<String>, // Отец матери
    pub mm: Option<String>, // Мать матери
    pub flock_name: Option<String>, // Flock Name
    pub herd_book_section: Option<String>, // Herd Book Section
    pub inbreeding: Option<String>, // Inbreeding
    pub keeper: Option<String>, // Keeper
    pub owner: Option<String>, // Owner
    pub weaned_progeny: Option<String>, // Weaned Progeny
    pub flocks_with_weaned_progeny: Option<String>, // Flocks with Weaned Progeny
    pub daughters_with_weaned_progeny: Option<String>, // Daughters with Weaned Progeny
    pub flocks_with_daughters_with_weaned_progeny: Option<String>, // Flocks with Daughters with Weaned Progeny
    pub pre_wean_direct: Option<String>, // Pre-Wean Direct
    pub pre_wean_maternal: Option<String>, // Pre-Wean Maternal
    pub wean_direct: Option<String>, // Wean Direct
    pub wean_maternal: Option<String>, // Wean Maternal
    pub pre_wean_combined: Option<String>, // Pre-Wean Combined
    pub wean_combined: Option<String>, // Wean Combined
    pub post_wean_direct: Option<String>, // Post-Wean Direct
    pub number_of_lambs_weaned: Option<String>, // Number of Lambs Weaned
    pub total_weight_weaned: Option<String>, // Total Weight Weaned
    pub age_at_first_lambing: Option<String>, // Age at First Lambing
    pub inter_lamb_period: Option<String>, // Inter-Lamb Period
    pub growth_index: Option<String>, // Growth Index
    pub reproduction_index: Option<String>, // Reproduction Index
    pub logix_merit_index: Option<String>, // Logix Merit Index
    pub dorper_logix_merit_index: Option<String>, // Dorper Logix Merit Index
    pub url_animal_card: Option<String>, // адрес карточки животного
    pub breed: String, // порода
    pub parse_card: i32, // Признак добавления данных из карточки 0 - нет, 1 - да
    pub fdm: Option<String>, // Flock Name
    pub name: Option<String>, // Identification Number
    pub comp_no: Option<String>, // S.A. Reg Number
    pub id_animal: Option<String>, // идентификатор животного, unique=True
    pub byear: Option<String>, // Birth Date
    pub inbred: Option<String>, // Inbreeding
    pub status: Option<String>, // статус животного
    pub sex: Option<String>, // пол
    pub lmi_1: Option<String>, // LMI
    pub lmi_2: Option<String>, // LMI
    pub lmi_3: Option<String>, // LMI
    pub lmi_4: Option<String>, // LMI
    pub growth_ind_1: Option<String>, // Growth Ind
    pub growth_ind_2: Option<String>, // Growth Ind
    pub growth_ind_3: Option<String>, // Growth Ind
    pub growth_ind_4: Option<String>, // Growth Ind
    pub reprod_ind_1: Option<String>, // Reprod. Ind
    pub reprod_ind_2: Option<String>, // Reprod. Ind
    pub reprod_ind_3: Option<String>, // Reprod. Ind
    pub reprod_ind_4: Option<String>, // Reprod. Ind
    pub wean_dir_1: Option<String>, // Wean Dir
    pub wean_dir_2: Option<String>, // Wean Dir
    pub wean_dir_3: Option<String>, // Wean Dir
    pub wean_dir_4: Option<String>, // Wean Dir
    pub wean_mat_1: Option<String>, // Wean Mat
    pub wean_mat_2: Option<String>, // Wean Mat
    pub wean_mat_3: Option<String>, // Wean Mat
    pub wean_mat_4: Option<String>, // Wean Mat
    pub wean_comb_1: Option<String>, // Wean Comb
    pub wean_comb_2: Option<String>, // Wean Comb
    pub wean_comb_3: Option<String>, // Wean Comb
    pub wean_comb_4: Option<String>, // Wean Comb
    pub n_lambs_weaned_1: Option<String>, // N Lambs Weaned
    pub n_lambs_weaned_2: Option<String>, // N Lambs Weaned
    pub n_lambs_weaned_3: Option<String>, // N Lambs Weaned
    pub n_lambs_weaned_4: Option<String>, // N Lambs Weaned
    pub tww_1: Option<String>, // TWW
    pub tww_2: Option<String>, // TWW
    pub tww_3: Option<String>, // TWW
    pub tww_4: Option<String>, // TWW
    pub afl_1: Option<String>, // AFL
    pub afl_2: Option<String>, // AFL
    pub afl_3: Option<String>, // AFL
    pub afl_4: Option<String>, // AFL
    pub ilp_1: Option<String>, // ILP
    pub ilp_2: Option<String>, // ILP
    pub ilp_3: Option<String>, // ILP
    pub ilp_4: Option<String>, // ILP
    pub create_at: String, // Create At
    pub update_at: String, // Update At
}

// impl AnimalData {
//     pub fn create_table(conn: &SqliteConnection) -> QueryResult<()> {
//         diesel::sql_query(
//             r#"
//                 CREATE TABLE IF NOT EXISTS animals (
//                     id INTEGER PRIMARY KEY AUTOINCREMENT,
//                     create_at TEXT NOT NULL,
//                     update_at TEXT NOT NULL,
//                     o TEXT,
//                     m TEXT,
//                     oo TEXT,
//                     mo TEXT,
//                     om TEXT,
//                     mm TEXT,
//                     flock_name TEXT,
//                     herd_book_section TEXT,
//                     inbreeding TEXT,
//                     keeper TEXT,
//                     owner TEXT,
//                     weaned_progeny TEXT,
//                     flocks_with_weaned_progeny TEXT,
//                     daughters_with_weaned_progeny TEXT,
//                     flocks_with_daughters_with_weaned_progeny TEXT,
//                     pre_wean_direct TEXT,
//                     pre_wean_maternal TEXT,
//                     wean_direct TEXT,
//                     wean_maternal TEXT,
//                     pre_wean_combined TEXT,
//                     wean_combined TEXT,
//                     post_wean_direct TEXT,
//                     number_of_lambs_weaned TEXT,
//                     total_weight_weaned TEXT,
//                     growth_ind_1 TEXT,
//                     growth_ind_2 TEXT,
//                     growth_ind_3 TEXT,
//                     growth_ind_4 TEXT,
//                     reprod_ind_1 TEXT,
//                     reprod_ind_2 TEXT,
//                     reprod_ind_3 TEXT,
//                     reprod_ind_4 TEXT,
//                     wean_dir_1 TEXT,
//                     wean_dir_2 TEXT,
//                     wean_dir_3 TEXT,
//                     wean_dir_4 TEXT,
//                     wean_mat_1 TEXT,
//                     wean_mat_2 TEXT,
//                     wean_mat_3 TEXT,
//                     wean_mat_4 TEXT,
//                     wean_comb_1 TEXT,
//                     wean_comb_2 TEXT,
//                     wean_comb_3 TEXT,
//                     wean_comb_4 TEXT,
//                     n_lambs_weaned_1 TEXT,
//                     n_lambs_weaned_2 TEXT,
//                     n_lambs_weaned_3 TEXT,
//                     n_lambs_weaned_4 TEXT,
//                     tww_1 TEXT,
//                     tww_2 TEXT,
//                     tww_3 TEXT,
//                     tww_4 TEXT,
//                     afl_1 TEXT,
//                     afl_2 TEXT,
//                     afl_3 TEXT,
//                     afl_4 TEXT,
//                     ilp_1 TEXT,
//                     ilp_2 TEXT,
//                     ilp_3 TEXT,
//                     ilp_4 TEXT
//                 )
//             "#
//         )
//         .execute(&conn)?;

//         Ok(())
//     }

/// Возвращает вектор строк с именами всех столбцов в таблице.
///
/// # Пример
///
/// ```
/// let field_names = get_all_names_column();
/// println!("{:?}", field_names);
/// ```
///
/// # Возвращает
///
/// `Vec<&'static str>` - вектор строк с именами столбцов.
pub fn get_all_names_column() -> Vec<&'static str> {
    let field_names = vec![
        "id",
        "create_at",
        "update_at",
        "o",
        "m",
        "oo",
        "mo",
        "om",
        "mm",
        "flock_name",
        "status",
        "herd_book_section",
        "inbreeding",
        "keeper",
        "owner",
        "weaned_progeny",
        "flocks_with_weaned_progeny",
        "daughters_with_weaned_progeny",
        "flocks_with_daughters_with_weaned_progeny",
        "pre_wean_direct",
        "pre_wean_maternal",
        "wean_direct",
        "wean_maternal",
        "pre_wean_combined",
        "wean_combined",
        "post_wean_direct",
        "number_of_lambs_weaned",
        "total_weight_weaned",
        "age_at_first_lambing",
        "inter_lamb_period",
        "growth_index",
        "reproduction_index",
        "logix_merit_index",
        "dorper_logix_merit_index",
        "url_animal_card",
        "breed",
        "parse_card",
        "fdm",
        "name",
        "comp_no",
        "id_animal",
        "byear",
        "inbred",
        "sex",
        "lmi_1",
        "lmi_2",
        "lmi_3",
        "lmi_4",
        "growth_ind_1",
        "growth_ind_2",
        "growth_ind_3",
        "growth_ind_4",
        "reprod_ind_1",
        "reprod_ind_2",
        "reprod_ind_3",
        "reprod_ind_4",
        "wean_dir_1",
        "wean_dir_2",
        "wean_dir_3",
        "wean_dir_4",
        "wean_mat_1",
        "wean_mat_2",
        "wean_mat_3",
        "wean_mat_4",
        "wean_comb_1",
        "wean_comb_2",
        "wean_comb_3",
        "wean_comb_4",
        "n_lambs_weaned_1",
        "n_lambs_weaned_2",
        "n_lambs_weaned_3",
        "n_lambs_weaned_4",
        "tww_1",
        "tww_2",
        "tww_3",
        "tww_4",
        "afl_1",
        "afl_2",
        "afl_3",
        "afl_4",
        "ilp_1",
        "ilp_2",
        "ilp_3",
        "ilp_4"
    ];
    field_names
}
/// Возвращает текущую дату и время в формате `%Y-%m-%d %H:%M:%S.%f`.
///
/// # Пример
///
/// ```rust
/// let datetime_str = AnimalsDataRow::get_current_datatime_repr();
/// println!("{}", datetime_str);
/// ```
pub fn get_current_datatime_repr() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S.%f").to_string()
}

// pub fn new(map: &HashMap<String, String>) -> Result<AnimalData, String> {
//     let create_at = Self::get_current_datatime_repr();
//     let update_at = map
//         .get("update_at")
//         .cloned()
//         .unwrap_or_else(|| create_at.clone());
//     let breed = map.get("breed").cloned().expect("Missing breed");
//     let parse_card = map
//         .get("parse_card")
//         .cloned()
//         .and_then(|s| s.parse().ok())
//         .unwrap_or(0);

//     let new_animal = AnimalData {
//         id: map.get("id").and_then(|v| v.parse().ok()),
//         o: map.get("o").cloned(), // Отец
//         m: map.get("m").cloned(), // Мать
//         oo: map.get("oo").cloned(), // Отец отца
//         mo: map.get("mo").cloned(), // Мать отца
//         om: map.get("om").cloned(), // Отец матери
//         mm: map.get("mm").cloned(), // Мать матери
//         flock_name: map.get("flock_name").cloned(), // Flock Name
//         herd_book_section: map.get("herd_book_section").cloned(), // Herd Book Section
//         inbreeding: map.get("inbreeding").cloned(), // Inbreeding
//         keeper: map.get("keeper").cloned(), // Keeper
//         owner: map.get("owner").cloned(), // Owner
//         weaned_progeny: map.get("weaned_progeny").cloned(), // Weaned Progeny
//         flocks_with_weaned_progeny: map.get("flocks_with_weaned_progeny").cloned(), // Flocks with Weaned Progeny
//         daughters_with_weaned_progeny: map.get("daughters_with_weaned_progeny").cloned(), // Daughters with Weaned Progeny
//         flocks_with_daughters_with_weaned_progeny: map
//             .get("flocks_with_daughters_with_weaned_progeny")
//             .cloned(), // Flocks with Daughters with Weaned Progeny
//         pre_wean_direct: map.get("pre_wean_direct").cloned(), // Pre-Wean Direct
//         pre_wean_maternal: map.get("pre_wean_maternal").cloned(), // Pre-Wean Maternal
//         wean_direct: map.get("wean_direct").cloned(), // Wean Direct
//         wean_maternal: map.get("wean_maternal").cloned(), // Wean Maternal
//         pre_wean_combined: map.get("pre_wean_combined").cloned(), // Pre-Wean Combined
//         wean_combined: map.get("wean_combined").cloned(), // Wean Combined
//         post_wean_direct: map.get("post_wean_direct").cloned(), // Post-Wean Direct
//         number_of_lambs_weaned: map.get("number_of_lambs_weaned").cloned(), // Number of Lambs Weaned
//         total_weight_weaned: map.get("total_weight_weaned").cloned(), // Total Weight Weaned
//         age_at_first_lambing: map.get("age_at_first_lambing").cloned(), // Age at First Lambing
//         inter_lamb_period: map.get("inter_lamb_period").cloned(), // Inter-Lamb Period
//         growth_index: map.get("growth_index").cloned(), // Growth Index
//         reproduction_index: map.get("reproduction_index").cloned(), // Reproduction Index
//         logix_merit_index: map.get("logix_merit_index").cloned(), // Logix Merit Index
//         dorper_logix_merit_index: map.get("dorper_logix_merit_index").cloned(), // Dorper Logix Merit Index
//         url_animal_card: map.get("url_animal_card").cloned(), // адрес карточки животного
//         breed, // порода
//         parse_card, // Признак добавления данных из карточки 0 - нет, 1 - да
//         fdm: map.get("fdm").cloned(), // Flock Name
//         name: map.get("name").cloned(), // Identification Number
//         comp_no: map.get("comp_no").cloned(), // S.A. Reg Number
//         id_animal: map.get("id_animal").cloned(), // идентификатор животного, unique=True
//         byear: map.get("byear").cloned(), // Birth Date
//         inbred: map.get("inbred").cloned(), // Inbreeding
//         status: map.get("status").cloned(), // статус животного
//         sex: map.get("sex").cloned(), // пол
//         lmi_1: map.get("lmi_1").cloned(), // LMI
//         lmi_2: map.get("lmi_2").cloned(), // LMI
//         lmi_3: map.get("lmi_3").cloned(), // LMI
//         lmi_4: map.get("lmi_4").cloned(), // LMI
//         growth_ind_1: map.get("growth_ind_1").cloned(), // Growth Ind
//         growth_ind_2: map.get("growth_ind_2").cloned(), // Growth Ind
//         growth_ind_3: map.get("growth_ind_3").cloned(), // Growth Ind
//         growth_ind_4: map.get("growth_ind_4").cloned(), // Growth Ind
//         reprod_ind_1: map.get("reprod_ind_1").cloned(), // Reprod. Ind
//         reprod_ind_2: map.get("reprod_ind_2").cloned(), // Reprod. Ind
//         reprod_ind_3: map.get("reprod_ind_3").cloned(), // Reprod. Ind
//         reprod_ind_4: map.get("reprod_ind_4").cloned(), // Reprod. Ind
//         wean_dir_1: map.get("wean_dir_1").cloned(), // Wean Dir
//         wean_dir_2: map.get("wean_dir_2").cloned(), // Wean Dir
//         wean_dir_3: map.get("wean_dir_3").cloned(), // Wean Dir
//         wean_dir_4: map.get("wean_dir_4").cloned(), // Wean Dir
//         wean_mat_1: map.get("wean_mat_1").cloned(), // Wean Mat
//         wean_mat_2: map.get("wean_mat_2").cloned(), // Wean Mat
//         wean_mat_3: map.get("wean_mat_3").cloned(), // Wean Mat
//         wean_mat_4: map.get("wean_mat_4").cloned(), // Wean Mat
//         wean_comb_1: map.get("wean_comb_1").cloned(), // Wean Comb
//         wean_comb_2: map.get("wean_comb_2").cloned(), // Wean Comb
//         wean_comb_3: map.get("wean_comb_3").cloned(), // Wean Comb
//         wean_comb_4: map.get("wean_comb_4").cloned(), // Wean Comb
//         n_lambs_weaned_1: map.get("n_lambs_weaned_1").cloned(), // N Lambs Weaned
//         n_lambs_weaned_2: map.get("n_lambs_weaned_2").cloned(), // N Lambs Weaned
//         n_lambs_weaned_3: map.get("n_lambs_weaned_3").cloned(), // N Lambs Weaned
//         n_lambs_weaned_4: map.get("n_lambs_weaned_4").cloned(), // N Lambs Weaned
//         tww_1: map.get("tww_1").cloned(), // TWW
//         tww_2: map.get("tww_2").cloned(), // TWW
//         tww_3: map.get("tww_3").cloned(), // TWW
//         tww_4: map.get("tww_4").cloned(), // TWW
//         afl_1: map.get("afl_1").cloned(), // AFL
//         afl_2: map.get("afl_2").cloned(), // AFL
//         afl_3: map.get("afl_3").cloned(), // AFL
//         afl_4: map.get("afl_4").cloned(), // AFL
//         ilp_1: map.get("ilp_1").cloned(), // ILP
//         ilp_2: map.get("ilp_2").cloned(), // ILP
//         ilp_3: map.get("ilp_3").cloned(), // ILP
//         ilp_4: map.get("ilp_4").cloned(), // ILP
//         create_at,
//         update_at,
//     };
//     Ok(new_animal)
// }

// pub fn add_data_to_database(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
//     let mut stmt = conn.prepare("SELECT id FROM animals_data WHERE comp_no = ?")?;
//     let existing_record = stmt
//         .query_row(&[&self.comp_no], |row| row.get(0))
//         .map_err(|e| rusqlite::Error::QueryReturnedNoRows)?;

//     if let Some(_) = existing_record {
//         // Запись уже существует, пропускаем вставку
//         return Ok(());
//     }

//     let mut stmt = conn.prepare(
//         "INSERT INTO animals_data (url_animal_card,
//         breed, parse_card, fdm, name, comp_no, id_animal, byear, inbred, status, sex, lmi_1, lmi_2, lmi_3, lmi_4, growth_ind_1, growth_ind_2, growth_ind_3, growth_ind_4,
//         reprod_ind_1, reprod_ind_2, reprod_ind_3, reprod_ind_4, wean_dir_1, wean_dir_2, wean_dir_3, wean_dir_4, wean_mat_1, wean_mat_2, wean_mat_3,
//         wean_mat_4, wean_comb_1, wean_comb_2, wean_comb_3, wean_comb_4, n_lambs_weaned_1, n_lambs_weaned_2, n_lambs_weaned_3, n_lambs_weaned_4, tww_1,
//         tww_2, tww_3, tww_4, afl_1, afl_2, afl_3, afl_4, ilp_1, ilp_2, ilp_3, ilp_4, create_at, update_at,) VALUES (?1,
//             ?2,
//             ?3,
//             ?4,
//             ?5,
//             ?6,
//             ?7,
//             ?8,
//             ?9)"
//     )?;
//     stmt.execute(
//         &[
//             &self.url_animal_card,
//             &self.breed.to_string(),
//             &self.parse_card,
//             &self.fdm,
//             &self.name,
//             &self.comp_no,
//             &self.id_animal,
//             &self.byear,
//             &self.inbred,
//             &self.status,
//             &self.sex,
//             &self.lmi_1,
//             &self.lmi_2,
//             &self.lmi_3,
//             &self.lmi_4,
//             &self.growth_ind_1,
//             &self.growth_ind_2,
//             &self.growth_ind_3,
//             &self.growth_ind_4,
//             &self.reprod_ind_1,
//             &self.reprod_ind_2,
//             &self.reprod_ind_3,
//             &self.reprod_ind_4,
//             &self.wean_dir_1,
//             &self.wean_dir_2,
//             &self.wean_dir_3,
//             &self.wean_dir_4,
//             &self.wean_mat_1,
//             &self.wean_mat_2,
//             &self.wean_mat_3,
//             &self.wean_mat_4,
//             &self.wean_comb_1,
//             &self.wean_comb_2,
//             &self.wean_comb_3,
//             &self.wean_comb_4,
//             &self.n_lambs_weaned_1,
//             &self.n_lambs_weaned_2,
//             &self.n_lambs_weaned_3,
//             &self.n_lambs_weaned_4,
//             &self.tww_1,
//             &self.tww_2,
//             &self.tww_3,
//             &self.tww_4,
//             &self.afl_1,
//             &self.afl_2,
//             &self.afl_3,
//             &self.afl_4,
//             &self.ilp_1,
//             &self.ilp_2,
//             &self.ilp_3,
//             &self.ilp_4,
//             &self.create_at.to_string(),
//             &self.update_at.to_string(),
//         ]
//     )?;

//     Ok(())
// }
// }
