#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod date;
mod note;
mod errors;

#[ink::contract]
mod todo {
    use ink::{
        storage::Mapping,
        prelude::{
            vec::Vec,
            string::String,
        }
    };

    use super::date::{
        DayOfWeek,
        Date,
        UncheckedDate,
        DateMethods,
    };

    use super::note::Note;

    use super::errors::TodoError;

    #[ink(storage)]
    pub struct Todo{
        notes: Mapping<(AccountId, Date), Vec<Note>>,
        repeating_notes: Mapping<(AccountId, DayOfWeek), Vec<Note>>,
        completed_repeating_notes: Mapping<(AccountId, Date), Vec<u64>>,
        note_id_counter: Mapping<AccountId, u64>,
    }

    impl Todo {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                notes: Default::default(),
                repeating_notes: Default::default(),
                completed_repeating_notes: Default::default(),
                note_id_counter: Default::default(),
            }
        }

        #[ink(message)]
        pub fn get_notes(&self, unchecked_date: UncheckedDate) -> 
        Result<Vec<Note>, TodoError> {
            let date = Date::from_unchecked(unchecked_date)?;
            let day_of_week = date.day_of_week()?;
            let account_id = self.env().caller();

            let notes = &mut self.notes.get((account_id, date)).unwrap_or_default();
            let repeating_notes = &mut self.repeating_notes.get((account_id, day_of_week)).unwrap_or_default();
            let completed_repeating_notes = self.completed_repeating_notes.get((account_id, date)).unwrap_or_default();

            let tmp_mapping: &mut Mapping<u64, bool> = &mut Mapping::new();

            completed_repeating_notes.iter().for_each(|note_id| {tmp_mapping.insert(note_id, &true);});

            repeating_notes.iter_mut().for_each(|note| {
                if tmp_mapping.get(note.id).unwrap_or(false) {
                    note.completed = true;
                }
            });
    
            notes.extend(repeating_notes.clone());

            Ok(notes.clone())
        }

        #[ink(message)]
        pub fn get_repeating_notes(&self, day_of_week: DayOfWeek) -> Result<Vec<Note>, TodoError> {
            let account_id = self.env().caller();
            Ok(self.repeating_notes.get((account_id, day_of_week)).unwrap_or_default())
        }

        #[ink(message)]
        pub fn add_note(&mut self, unchecked_date: UncheckedDate, title: String, description: String) -> Result<u64, TodoError> {
            let date = Date::from_unchecked(unchecked_date)?;
            let account_id = self.env().caller();

            if date.compare(self.today()) < 0 {
                return Err(TodoError::CantAddNoteEarlierThanToday);
            }

            let notes = &mut self.notes.get((&account_id, &date)).unwrap_or_default();

            let note_id = self.add_any_note(notes, &account_id, title, description, false);

            self.notes.insert((account_id, date), notes);

            Ok(note_id)
        }

        #[ink(message)]
        pub fn add_repeating_note(&mut self, day_of_week: DayOfWeek, title: String, description: String) -> u64 {
            let account_id = self.env().caller();

            let notes = &mut self.repeating_notes.get((account_id, &day_of_week)).unwrap_or_default();

            let note_id = self.add_any_note(notes, &account_id, title, description, true);

            self.repeating_notes.insert((account_id, &day_of_week), notes);

            note_id
        }

        #[ink(message)]
        pub fn edit_note(&mut self, unchecked_date: UncheckedDate, note_id: u64, title: String, descirption: String) -> Result<(), TodoError> {
            let account_id = self.env().caller();
            let date = Date::from_unchecked(unchecked_date)?;

            if date.compare(self.today()) <= 0 {
                return Err(TodoError::CantEditTodayOrEarlierNotes);
            }

            let notes = &mut self.notes.get((&account_id, &date)).unwrap_or_default();

            Self::edit_any_note(notes, note_id, title, descirption)?;

            self.notes.insert((&account_id, &date), notes);

            Ok(())
        }

        #[ink(message)]
        pub fn edit_repeating_note(&mut self, day_of_week: DayOfWeek, note_id: u64, title: String, descirption: String) -> Result<(), TodoError> {
            let account_id = self.env().caller();
            let notes = &mut self.repeating_notes.get((&account_id, &day_of_week)).unwrap_or_default();

            Self::edit_any_note(notes, note_id, title, descirption)?;

            self.repeating_notes.insert((&account_id, &day_of_week), notes);

            Ok(())
        }

        #[ink(message)]
        pub fn delete_note(&mut self, unchecked_date: UncheckedDate, note_id: u64) -> Result<(), TodoError> {
            let account_id = self.env().caller();
            let date = Date::from_unchecked(unchecked_date)?;

            if date.compare(self.today()) <= 0 {
                return Err(TodoError::CantDeleteTodayOrEarlierNotes);
            }

            let notes = &mut self.notes.get((&account_id, &date)).unwrap_or_default();

            Self::delete_any_note(notes, note_id)?;

            self.notes.insert((&account_id, &date), notes);

            Ok(())
        }

        #[ink(message)]
        pub fn delete_repeating_note(&mut self, day_of_week: DayOfWeek, note_id: u64) -> Result<(), TodoError> {
            let account_id = self.env().caller();
            let notes = &mut self.repeating_notes.get((&account_id, &day_of_week)).unwrap_or_default();

            Self::delete_any_note(notes, note_id)?;

            self.repeating_notes.insert((&account_id, &day_of_week), notes);

            Ok(())
        }

        #[ink(message)]
        pub fn complete_note(&mut self, unchecked_date: UncheckedDate, note_id: u64) -> Result<(), TodoError> {
            let account_id = self.env().caller();
            let date = Date::from_unchecked(unchecked_date)?;
            let day_of_week = date.day_of_week()?;

            if !self.is_today(date) {
                return Err(TodoError::CantCompleteNoteNotFromToday);
            }

            let notes = &mut self.notes.get((&account_id, &date)).unwrap_or_default();

            let repeating_notes = &mut self.repeating_notes.get((&account_id, &day_of_week)).unwrap_or_default();

            let note = notes.iter_mut().find(|note| {note.id == note_id});

            let note_repeating = repeating_notes.iter_mut().find(|note| {note.id == note_id});

            if note.is_some() {
                note.unwrap().completed = true;
                self.notes.insert((&account_id, &date), notes);
            } 
            else if note_repeating.is_some() {
                let completed_repeating = &mut self.completed_repeating_notes.get((&account_id, &date)).unwrap_or_default();

                completed_repeating.push(note_repeating.unwrap().id);

                self.completed_repeating_notes.insert((&account_id, &date), completed_repeating);
            } 
            else {
                return Err(TodoError::NoteIdDoesntExist);
            }

            Ok(())
        } 

        #[ink(message)]
        pub fn get_block_timestamp(&self) -> u64 {
            self.env().block_timestamp()
        }

        #[ink(message)]
        pub fn today(&self) -> Date {
            Date::from_timestamp(self.env().block_timestamp())
        }

        #[ink(message)]
        pub fn is_today(&self, date: Date) -> bool {
            let today = self.today();
            today.eq(&date)
        }

        fn add_any_note(&mut self, notes: &mut Vec<Note>, account_id: &AccountId, title: String, description: String, is_repeating: bool) -> u64 {
            let note_id = self.note_id_counter.get(account_id).unwrap_or_default();
            let note = Note::new(note_id, title, description, is_repeating);

            notes.push(note);

            self.note_id_counter.insert(account_id, &(note_id + 1));

            note_id
        }

        fn edit_any_note(notes: &mut Vec<Note>, note_id: u64, title: String, descirption: String) -> Result<(), TodoError> {
            let note = notes.iter_mut().find(|note| note.id == note_id).ok_or(TodoError::NoteIdDoesntExist)?;

            note.title = title;
            note.description = descirption;

            Ok(())
        }

        fn delete_any_note(notes: &mut Vec<Note>, note_id: u64) -> Result<(), TodoError> {
            let idx = notes.iter().position(|note| note.id == note_id).ok_or(TodoError::NoteIdDoesntExist)?;
            notes.remove(idx);

            Ok(())
        }
    } 

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn get_notes_works() {
            let todo = &Todo::new();

            let notes = todo.get_notes((2024, 1, 1)).expect("Get Notes fails");

            assert_eq!(notes.len(), 0, "Size of notes is not 0");
        }

        #[ink::test]
        fn add_and_get_notes_works() {
            let todo = &mut Todo::new();

            let title = String::from("Title");
            let description = String::from("Description");

            let today = todo.today();

            let yesterday = today.subtract_days(1);
            let seven_days_after = today.add_days(7);
            let seven_days_ago = today.subtract_days(7);

            let today_day_of_week = today.day_of_week().expect("Invalid date");

            let notes_today_before_insert = todo.get_notes(today).expect("Get notes fails");

            let note_1_today_id = todo.add_note(today, title.clone(), description.clone()).expect("Add notes fails");
            let note_repeating_today_id = todo.add_repeating_note(today_day_of_week, title.clone(), description.clone());
            let note_2_today_id = todo.add_note(today, title.clone(), description.clone()).expect("Add notes fails");
            let note_repeating_seven_days_after_id = todo.add_repeating_note(seven_days_after.day_of_week().unwrap(), title.clone(), description.clone());
            let result_add_yesterday = todo.add_note(yesterday, title.clone(), description.clone());

            let notes_today_after_insert = todo.get_notes(today).expect("Get notes fails");
            let notes_seven_days_ago = todo.get_notes(seven_days_ago).expect("Get notes fails");
            let notes_seven_days_after = todo.get_notes(seven_days_after).expect("Get notes fails");
            let notes_yesterday = todo.get_notes(yesterday).expect("Get notes fails");

            assert_eq!(Err(TodoError::CantAddNoteEarlierThanToday), result_add_yesterday);

            assert_eq!(notes_today_before_insert.len(), 0);
            assert_eq!(notes_today_after_insert.len(), 4);
            assert_eq!(notes_seven_days_ago.len(), 2);
            assert_eq!(notes_seven_days_after.len(), 2);
            assert_eq!(notes_yesterday.len(), 0);

            assert!(notes_today_after_insert.get(0).unwrap_or(&Note::default()).eq(&Note {
                id: note_1_today_id,
                completed: false,
                title: title.clone(),
                description: description.clone(),
                is_repeating: false,
            }));

            assert!(notes_today_after_insert.get(1).unwrap_or(&Note::default()).eq(&Note {
                id: note_2_today_id,
                completed: false,
                title: title.clone(),
                description: description.clone(),
                is_repeating: false,
            }));

            assert!(notes_today_after_insert.get(2).unwrap_or(&Note::default()).eq(&Note {
                id: note_repeating_today_id,
                completed: false,
                title: title.clone(),
                description: description.clone(),
                is_repeating: true,
            }));

            assert!(notes_today_after_insert.get(3).unwrap_or(&Note::default()).eq(&Note {
                id: note_repeating_seven_days_after_id,
                completed: false,
                title: title.clone(),
                description: description.clone(),
                is_repeating: true,
            }));

            assert!(notes_seven_days_ago.get(0).unwrap_or(&Note::default()).eq(&Note {
                id: note_repeating_today_id,
                completed: false,
                title: title.clone(),
                description: description.clone(),
                is_repeating: true,
            }));

            assert!(notes_seven_days_ago.get(1).unwrap_or(&Note::default()).eq(&Note {
                id: note_repeating_seven_days_after_id,
                completed: false,
                title: title.clone(),
                description: description.clone(),
                is_repeating: true,
            }));
        }
    
        #[ink::test]
        fn edit_note_works() {
            let todo = &mut Todo::new();

            let title = String::from("First note");
            let description = String::from("First edited note");

            let edited_title = String::from("First edited note");
            let edited_description = String::from("First edit created note");

            let today = todo.today();
            let tomorrow = today.add_days(1);
            let yesterday = today.subtract_days(1);

            let dummy_id = 0;

            let note_today_id = todo.add_note(today, title.clone(), description.clone()).expect("Add note fails");
            let note_tomorrow_id = todo.add_note(tomorrow, title.clone(), description.clone()).expect("Add note fails");

            let edit_result_today = todo.edit_note(today, note_today_id, edited_title.clone(), edited_description.clone());
            let edit_result_tomorrow = todo.edit_note(tomorrow, note_tomorrow_id, edited_title.clone(), edited_description.clone());
            let edit_result_yesterday = todo.edit_note(yesterday, dummy_id, edited_title.clone(), edited_description.clone());

            let notes_today = todo.get_notes(today).unwrap_or_default();
            let notes_tomorrow = todo.get_notes(tomorrow).unwrap_or_default();

            assert_eq!(Err(TodoError::CantEditTodayOrEarlierNotes), edit_result_today);
            assert_eq!(Ok(()), edit_result_tomorrow);
            assert_eq!(Err(TodoError::CantEditTodayOrEarlierNotes), edit_result_yesterday);

            assert!(notes_today.get(0).unwrap_or(&Note::default()).eq(&Note {
                id: note_today_id,
                completed: false,
                title: title.clone(),
                description: description.clone(),
                is_repeating: false,
            }));
            assert!(notes_tomorrow.get(0).unwrap_or(&Note::default()).eq(&Note {
                id: note_tomorrow_id,
                completed: false,
                title: edited_title.clone(),
                description: edited_description.clone(),
                is_repeating: false,
            }));

            let result = todo.edit_note(tomorrow, note_today_id, edited_title.clone(), edited_description.clone());

            assert_eq!(result, Err(TodoError::NoteIdDoesntExist), "Did not throw note id doesnt exist error");
        }

        #[ink::test]
        fn edit_repeating_note_works() {
            let todo = &mut Todo::new();

            let title = String::from("First note");
            let description = String::from("First edited note");

            let today = todo.today();
            let today_day_of_week = today.day_of_week().unwrap();

            let edited_title = String::from("First edited note");
            let edited_description = String::from("First edit created note");

            let note_id = todo.add_repeating_note(today_day_of_week.clone(), title.clone(), description.clone());

            let notes = todo.get_notes(today).unwrap_or_default();

            assert!(notes.get(0).unwrap_or(&Note::default()).eq(&Note {
                id: note_id,
                completed: false,
                title: title.clone(),
                description: description.clone(),
                is_repeating: true,
            }));

            todo.edit_repeating_note(today_day_of_week.clone(), note_id, edited_title.clone(), edited_description.clone()).expect("Edit note fails");

            let notes = todo.get_notes(today).unwrap_or_default();

            assert!(notes.get(0).unwrap_or(&Note::default()).eq(&Note {
                id: note_id,
                completed: false,
                title: edited_title.clone(),
                description: edited_description.clone(),
                is_repeating: true,
            }));

            let result = todo.edit_repeating_note(today_day_of_week.clone(), note_id + 1, edited_title.clone(), edited_description.clone());

            assert_eq!(result, Err(TodoError::NoteIdDoesntExist));
        }

        #[ink::test]
        fn delete_note_works() {
            let todo = &mut Todo::new();

            let title = String::from("First Note");
            let description = String::from("First created note");

            let today = todo.today();
            let tomorrow = today.add_days(1);
            let yesterday = today.subtract_days(2);

            let dummy_id = 0;

            let note_today_id = todo.add_note(today, title.clone(), description.clone()).expect("Add note fails");
            let note_tomorrow_id = todo.add_note(tomorrow, title.clone(), description.clone()).expect("Add note fails");

            let delete_result_today = todo.delete_note(today, note_today_id);
            let delete_result_tomorrow = todo.delete_note(tomorrow, note_tomorrow_id);
            let delete_result_yesterday = todo.delete_note(yesterday, dummy_id);

            let notes_today = todo.get_notes(today).expect("Get notes fails");
            let notes_tomorrow = todo.get_notes(tomorrow).expect("Get notes fails");

            assert_eq!(Err(TodoError::CantDeleteTodayOrEarlierNotes), delete_result_today);
            assert_eq!(Ok(()), delete_result_tomorrow);
            assert_eq!(Err(TodoError::CantDeleteTodayOrEarlierNotes), delete_result_yesterday);

            assert_eq!(notes_today.len(), 1);
            assert_eq!(notes_tomorrow.len(), 0);

            let result = todo.delete_note(tomorrow, note_today_id);

            assert_eq!(result, Err(TodoError::NoteIdDoesntExist));
        }

        #[ink::test]
        fn delete_repeating_note_works() {
            let todo = &mut Todo::new();

            let title = String::from("First Note");
            let description = String::from("First created note");

            let today = todo.today();
            let today_day_of_week = today.day_of_week().expect("Date::today() gives invalid dates");

            let today_repeating_note_id = todo.add_repeating_note(today_day_of_week.clone(), title.clone(), description.clone());
            let notes = todo.get_notes(today).unwrap_or_default();

            assert_eq!(notes.len(), 1);

            assert!(notes.get(0).unwrap_or(&Note::default()).eq(&Note {
                id: today_repeating_note_id,
                completed: false,
                title: title.clone(),
                description: description.clone(),
                is_repeating: true,
            }));

            todo.delete_repeating_note(today_day_of_week.clone(), today_repeating_note_id).expect("Delete note fails");
            let notes = todo.get_notes(today).expect("Get notes fails");

            assert_eq!(notes.len(), 0);

            let result = todo.delete_note(today.add_days(1), 6);

            assert_eq!(result, Err(TodoError::NoteIdDoesntExist));
        }

        #[ink::test]
        fn complete_notes_works() {
            let title = String::from("Some title");
            let description = String::from("Some description");

            let todo = &mut Todo::new();

            let today = todo.today();
            let tomorrow = today.add_days(1);
            let yesterday = today.subtract_days(1);

            let today_day_of_week = today.day_of_week().expect("Date::today() gives invalid date");
            let tomorrow_day_of_week = tomorrow.day_of_week().expect("Date::add_days(1) gives invalid date");
            let yesterday_day_of_week = yesterday.day_of_week().expect("Date::subtract_days(1) gives invalid date");

            let dummy_note = 0;

            let note_today_id = todo.add_note(today, title.clone(), description.clone()).expect("Add today note fails");
            let note_tomorrow_id = todo.add_note(tomorrow, title.clone(), description.clone()).expect("Add tomorrow note fails");

            let note_today_day_of_week_id = todo.add_repeating_note(today_day_of_week, title.clone(), description.clone());
            let note_tomorrow_day_of_week_id = todo.add_repeating_note(tomorrow_day_of_week, title.clone(), description.clone());
            let note_yesterday_day_of_week_id = todo.add_repeating_note(yesterday_day_of_week, title.clone(), description.clone());

            let result_today = todo.complete_note(today, note_today_id);
            let result_tomorrow = todo.complete_note(tomorrow, note_tomorrow_id);
            let result_yesterday = todo.complete_note(yesterday, dummy_note);

            let result_today_day_of_week = todo.complete_note(today, note_today_day_of_week_id);
            let result_tomorrow_day_of_week = todo.complete_note(tomorrow, note_tomorrow_day_of_week_id);
            let result_yesterday_day_of_week = todo.complete_note(yesterday, note_yesterday_day_of_week_id);

            let notes_today = todo.get_notes(today).expect("Get notes fails");
            let notes_tomorrow = todo.get_notes(tomorrow).expect("Get notes fails");
            let notes_yesterday = todo.get_notes(yesterday).expect("Get notes fails");

            assert_eq!(Ok(()), result_today);
            assert_eq!(Err(TodoError::CantCompleteNoteNotFromToday), result_tomorrow);
            assert_eq!(Err(TodoError::CantCompleteNoteNotFromToday), result_yesterday);
            assert_eq!(Ok(()), result_today_day_of_week);
            assert_eq!(Err(TodoError::CantCompleteNoteNotFromToday), result_tomorrow_day_of_week);
            assert_eq!(Err(TodoError::CantCompleteNoteNotFromToday), result_yesterday_day_of_week);

            assert!(notes_today.get(0).unwrap_or(&Note::default()).eq(&Note {
                id: note_today_id,
                completed: true,
                title: title.clone(),
                description: description.clone(),
                is_repeating: false,
            }));
            assert!(notes_today.get(1).unwrap_or(&Note::default()).eq(&Note {
                id: note_today_day_of_week_id,
                completed: true,
                title: title.clone(),
                description: description.clone(),
                is_repeating: true,
            }));

            assert!(notes_tomorrow.get(0).unwrap_or(&Note::default()).eq(&Note {
                id: note_tomorrow_id,
                completed: false,
                title: title.clone(),
                description: description.clone(),
                is_repeating: false,
            }));
            assert!(notes_tomorrow.get(1).unwrap_or(&Note::default()).eq(&Note {
                id: note_tomorrow_day_of_week_id,
                completed: false,
                title: title.clone(),
                description: description.clone(),
                is_repeating: true,
            }));

            assert!(notes_yesterday.get(0).unwrap_or(&Note::default()).eq(&Note {
                id: note_yesterday_day_of_week_id,
                completed: false,
                title: title.clone(),
                description: description.clone(),
                is_repeating: true,
            }));
        }
    }
}