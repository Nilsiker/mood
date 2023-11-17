use std::{
    error::Error,
    fs::{File, OpenOptions},
};

use chrono::NaiveDate;
use ron::{de::from_reader, ser::to_writer};
use serde::{Deserialize, Serialize};

use crate::{config::MoodConfig, error::MoodError, rating::Rating};

pub const DEFAULT_JOURNAL_NAME: &str = "journal.ron";

#[derive(Debug)]
pub enum JournalError {
    InvalidDateRange,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Journal {
    data: Vec<JournalEntry>,
}

impl Journal {
    pub fn init(MoodConfig { journal_path }: &MoodConfig) -> Result<Self, Box<dyn Error>> {
        if !journal_path
            .parent()
            .expect("journal path has a parent folder")
            .exists()
        {
            std::fs::create_dir_all(
                journal_path
                    .parent()
                    .expect("journal path has a parent folder"),
            )?;
        }

        let journal = match journal_path.is_file() {
            true => {
                let file = File::open(journal_path)?;
                let journal: Journal = from_reader(file).map_err(|e| {
                    MoodError::JournalFileError(format!("Read from file. I/O error: {e:?}"))
                })?;

                journal
            }
            false => {
                let journal = Journal::default();
                File::create(journal_path)?;
                journal
            }
        };
        Ok(journal)
    }

    pub fn save(&self, MoodConfig { journal_path }: &MoodConfig) -> Result<(), Box<dyn Error>> {
        let file = match journal_path.is_file() {
            true => OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(journal_path)?,
            false => File::create(journal_path)?,
        };

        to_writer(file, self)
            .map_err(|e| MoodError::JournalFileError(format!("Write to file. I/O error: {e:?}")))?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add_entry(&mut self, entry: JournalEntry) -> Option<JournalEntry> {
        match self
            .data
            .binary_search_by(|probe| probe.date.cmp(&entry.date))
        {
            Ok(index) => Some(std::mem::replace(&mut self.data[index], entry)),
            Err(index) => {
                self.data.insert(index, entry);
                None
            }
        }
    }

    pub fn get(&self, date: &NaiveDate) -> Option<&JournalEntry> {
        match self.data.binary_search_by(|probe| probe.date.cmp(date)) {
            Ok(v) => Some(&self.data[v]),
            Err(_) => None,
        }
    }

    pub fn get_entries(
        &self,
        from: &NaiveDate,
        to: &NaiveDate,
    ) -> Result<Vec<&JournalEntry>, JournalError> {
        if self.is_empty() {
            return Ok(vec![]);
        }
        if to < from {
            return Err(JournalError::InvalidDateRange);
        }

        let entries = self
            .data
            .iter()
            .filter(|entry| entry.date >= *from && entry.date <= *to)
            .collect::<Vec<&JournalEntry>>();

        Ok(entries)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct JournalEntry {
    pub date: NaiveDate,
    pub mood: Rating,
    pub note: String,
}

impl JournalEntry {
    pub fn new(date: NaiveDate, mood: Rating, note: String) -> Self {
        JournalEntry { date, mood, note }
    }
}

impl From<(NaiveDate, Rating, String)> for JournalEntry {
    fn from((date, mood, note): (NaiveDate, Rating, String)) -> Self {
        JournalEntry::new(date, mood, note)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Days;

    use crate::{helpers::today, journal::Journal, rating::Rating};

    use super::JournalEntry;

    #[test]
    fn add_entries_to_journal() {
        let mut journal = Journal::default();

        assert!(journal.len() == 0);

        let first = journal.add_entry(JournalEntry {
            date: today(),
            mood: Rating::Neutral,
            note: String::from("Was alright"),
        });

        assert!(first.is_none());
        assert!(journal.len() == 1);

        let second = journal.add_entry(JournalEntry {
            date: today()
                .checked_add_days(Days::new(1))
                .expect("should be able to increment day by one"),
            mood: Rating::Neutral,
            note: String::from("Was alright"),
        });

        assert!(second.is_none());
        assert!(journal.len() == 2);
    }

    #[test]
    fn add_duplicate_entry_to_journal() {
        let mut journal = Journal::default();
        let first_add = journal.add_entry(JournalEntry {
            date: today(),
            mood: Rating::Neutral,
            note: String::from("Was alright"),
        });

        assert!(first_add.is_none());

        let second_add = journal.add_entry(JournalEntry {
            date: today(),

            mood: Rating::Neutral,
            note: String::from("Was alright"),
        });

        assert!(second_add.is_some());
    }

    #[test]
    fn get_entries() {
        let mut journal = Journal::default();

        let yesterday = today().checked_sub_days(Days::new(1)).unwrap();
        let now = today();
        let tomorrow = today().checked_add_days(Days::new(1)).unwrap();
        let day_after_tomorrow = today().checked_add_days(Days::new(2)).unwrap();

        let yesterday_entry = JournalEntry {
            date: yesterday,
            mood: Rating::Bad,
            note: String::from("Yesterday"),
        };
        let now_entry = JournalEntry {
            date: now,
            mood: Rating::Neutral,
            note: String::from("Today"),
        };
        let day_after_tomorrow_entry = JournalEntry {
            date: day_after_tomorrow,
            mood: Rating::Great,
            note: String::from("Day after tomorrow"),
        };

        // Add in "wrong order"
        journal.add_entry(day_after_tomorrow_entry);
        journal.add_entry(now_entry.clone());
        journal.add_entry(yesterday_entry.clone());

        let entries = &journal.get_entries(&yesterday, &tomorrow).unwrap();

        fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
            let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
            matching == a.len() && matching == b.len()
        }

        let to_cmp = &vec![&yesterday_entry, &now_entry];
        assert!(do_vecs_match(entries, to_cmp));
    }

    #[test]
    fn get_entries_invalid_range() {
        let mut journal = Journal::default();

        let yesterday = today().checked_sub_days(Days::new(1)).unwrap();
        let now = today();
        let tomorrow = today().checked_add_days(Days::new(1)).unwrap();
        let day_after_tomorrow = today().checked_add_days(Days::new(2)).unwrap();

        let yesterday_entry = JournalEntry {
            date: yesterday,
            mood: Rating::Bad,
            note: String::from("Yesterday"),
        };
        let now_entry = JournalEntry {
            date: now,
            mood: Rating::Neutral,
            note: String::from("Today"),
        };
        let day_after_tomorrow_entry = JournalEntry {
            date: day_after_tomorrow,
            mood: Rating::Great,
            note: String::from("Day after tomorrow"),
        };

        // Add in "wrong order"
        journal.add_entry(day_after_tomorrow_entry);
        journal.add_entry(now_entry);
        journal.add_entry(yesterday_entry);

        assert!(journal.get_entries(&tomorrow, &yesterday).is_err());
    }
}
