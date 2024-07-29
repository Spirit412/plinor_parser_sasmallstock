/* 
Этот файл содержит код для определения моделей или структур базы данных. 
Он определяет структуру таблиц или коллекций в вашей базе данных и соответствующие поля.
*/

use rusqlite::{ params, Connection, Result };
use chrono::{ DateTime, Utc };

#[derive(Debug)]
pub struct AnimalsDataRow {
    // Primary key
    pub id: Option<i32>, // id

    // Timestamps
    pub create_at: String, // Create At
    pub update_at: String, // Update At

    // pedigree
    pub o: Option<String>, // Отец
    pub m: Option<String>, // Мать
    pub oo: Option<String>, // Отец отца
    pub mo: Option<String>, // Мать отца
    pub om: Option<String>, // Отец матери
    pub mm: Option<String>, // Мать матери
    // first_table_animal_card
    pub flock_name: Option<String>, // Flock Name
    pub status: Option<String>, // Status
    pub herd_book_section: Option<String>, // Herd Book Section
    pub inbreeding: Option<String>, // Inbreeding
    pub keeper: Option<String>, // Keeper
    pub owner: Option<String>, // Owner
    pub weaned_progeny: Option<String>, // Weaned Progeny
    pub flocks_with_weaned_progeny: Option<String>, // Flocks with Weaned Progeny
    pub daughters_with_weaned_progeny: Option<String>, // Daughters with Weaned Progeny
    pub flocks_with_daughters_with_weaned_progeny: Option<String>, // Flocks with Daughters with Weaned Progeny
    // breeding_values
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
    // selection_indices
    pub growth_index: Option<String>, // Growth Index
    pub reproduction_index: Option<String>, // Reproduction Index
    pub logix_merit_index: Option<String>, // Logix Merit Index
    pub dorper_logix_merit_index: Option<String>, // Dorper Logix Merit Index
    pub url_animal_card: Option<String>, // адрес карточки животного
    pub breed: String, // порода
    pub parse_card: i32, // Признак добавления данных из карточки
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
}

impl AnimalsDataRow {
    pub fn create_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS animals_data (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                create_at TEXT NOT NULL,
                update_at TEXT NOT NULL,
                o TEXT,
                m TEXT,
                oo TEXT,
                mo TEXT,
                om TEXT,
                mm TEXT,
                flock_name TEXT,
                status TEXT,
                herd_book_section TEXT,
                inbreeding TEXT,
                keeper TEXT,
                owner TEXT,
                weaned_progeny TEXT,
                flocks_with_weaned_progeny TEXT,
                daughters_with_weaned_progeny TEXT,
                flocks_with_daughters_with_weaned_progeny TEXT,
                pre_wean_direct TEXT,
                pre_wean_maternal TEXT,
                wean_direct TEXT,
                wean_maternal TEXT,
                pre_wean_combined TEXT,
                wean_combined TEXT,
                post_wean_direct TEXT,
                number_of_lambs_weaned TEXT,
                total_weight_weaned TEXT,
                age_at_first_lambing TEXT,
                inter_lamb_period TEXT,
                growth_index TEXT,
                reproduction_index TEXT,
                logix_merit_index TEXT,
                dorper_logix_merit_index TEXT,
                url_animal_card TEXT,
                breed TEXT NOT NULL,
                parse_card INTEGER DEFAULT 0,
                fdm TEXT,
                name TEXT,
                comp_no TEXT,
                id_animal TEXT UNIQUE,
                byear TEXT,
                inbred TEXT,
                status TEXT,
                sex TEXT,
                lmi_1 TEXT,
                lmi_2 TEXT,
                lmi_3 TEXT,
                lmi_4 TEXT,
                growth_ind_1 TEXT,
                growth_ind_2 TEXT,
                growth_ind_3 TEXT,
                growth_ind_4 TEXT,
                reprod_ind_1 TEXT,
                reprod_ind_2 TEXT,
                reprod_ind_3 TEXT,
                reprod_ind_4 TEXT,
                wean_dir_1 TEXT,
                wean_dir_2 TEXT,
                wean_dir_3 TEXT,
                wean_dir_4 TEXT,
                wean_mat_1 TEXT,
                wean_mat_2 TEXT,
                wean_mat_3 TEXT,
                wean_mat_4 TEXT,
                wean_comb_1 TEXT,
                wean_comb_2 TEXT,
                wean_comb_3 TEXT,
                wean_comb_4 TEXT,
                n_lambs_weaned_1 TEXT,
                n_lambs_weaned_2 TEXT,
                n_lambs_weaned_3 TEXT,
                n_lambs_weaned_4 TEXT,
                tww_1 TEXT,
                tww_2 TEXT,
                tww_3 TEXT,
                tww_4 TEXT,
                afl_1 TEXT,
                afl_2 TEXT,
                afl_3 TEXT,
                afl_4 TEXT,
                ilp_1 TEXT,
                ilp_2 TEXT,
                ilp_3 TEXT,
                ilp_4 TEXT
            )",
            []
        )?;
        Ok(())
    }

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
            "status",
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
    /// Возвращает текущую дату и время в формате `%Y-%m-%d %H:%M:%S.%f` для использования в качестве времени обновления.
    ///
    /// # Пример
    ///
    /// ```rust
    /// let animal = AnimalsDataRow {
    ///     // Инициализация полей
    /// };
    /// let next_update = animal.next_update_at();
    /// println!("{}", next_update);
    /// ```
    pub fn next_update_at(&self) -> String {
        Self::get_current_datatime_repr()
    }

    /// Обновляет поля структуры `AnimalsDataRow` на основе переданного словаря и устанавливает новое время обновления.
    ///
    /// # Аргументы
    ///
    /// * `data` - Словарь с новыми значениями для обновления полей.
    ///
    /// # Пример
    ///
    /// ```rust
    /// let mut animal = AnimalsDataRow {
    ///     // Инициализация полей
    /// };
    /// let data = vec![
    ///     ("status", "new_status"),
    ///     ("breed", "new_breed"),
    /// ].into_iter().collect::<HashMap<&str, &str>>();
    /// animal.update(data);
    /// ```
    pub fn update(&mut self, data: HashMap<&str, &str>) {
        self.update_at = Self::get_current_datatime_repr();
        for (key, value) in data {
            if
                let Some(field) = Self::get_all_names_column()
                    .iter()
                    .find(|&&f| f == key)
            {
                match *field {
                    "id" => {
                        self.id = value.parse().ok();
                    }
                    "create_at" => {
                        self.create_at = value.to_string();
                    }
                    "update_at" => {
                        self.update_at = value.to_string();
                    }
                    // Остальные поля опущены для краткости
                    _ => {}
                }
            }
        }
    }
}
