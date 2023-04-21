use std::{error::Error, fs::File};

use chrono::NaiveDate;
use ron::{de::from_reader, ser::to_writer};
use serde::{Deserialize, Serialize};

use crate::rating::Rating;

#[derive(Debug)]
pub enum JournalError {
    InvalidDateRange,
}

pub fn save_journal(journal: &Journal) -> Result<(), Box<dyn Error>> {
    let file = File::create("journal.ron")?;
    to_writer(file, journal)?;
    Ok(())
}
pub fn load_journal() -> Result<Journal, Box<dyn Error>> {
    let file = File::open("journal.ron")?;
    let journal = from_reader(file)?;
    Ok(journal)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Journal {
    data: Vec<JournalEntry>,
}

impl Journal {
    pub fn len(&self) -> usize {
        self.data.len()
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
    ) -> Result<&[JournalEntry], JournalError> {
        if self.len() == 0 {
            return Ok(&[]);
        }
        let from_index = match self.data.binary_search_by(|probe| probe.date.cmp(from)) {
            Ok(v) => v,
            Err(v) => v,
        };

        let to_index = match self.data.binary_search_by(|probe| probe.date.cmp(to)) {
            Ok(v) => v,
            Err(v) => v.saturating_sub(1),
        };

        match to_index < from_index {
            true => Err(JournalError::InvalidDateRange),
            false => Ok(&self.data[from_index..=to_index]),
        }
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
    use mood::today;

    use crate::{journal::Journal, rating::Rating};

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
            date: yesterday.clone(),
            mood: Rating::Bad,
            note: String::from("Yesterday"),
        };
        let now_entry = JournalEntry {
            date: now.clone(),
            mood: Rating::Neutral,
            note: String::from("Today"),
        };
        let day_after_tomorrow_entry = JournalEntry {
            date: day_after_tomorrow.clone(),
            mood: Rating::Great,
            note: String::from("Day after tomorrow"),
        };

        // Add in "wrong order"
        journal.add_entry(day_after_tomorrow_entry.clone());
        journal.add_entry(now_entry.clone());
        journal.add_entry(yesterday_entry.clone());

        let slice = journal.get_entries(&yesterday, &tomorrow).unwrap();

        fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
            let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
            matching == a.len() && matching == b.len()
        }

        let to_cmp = &vec![yesterday_entry, now_entry];
        assert!(do_vecs_match(&slice.to_vec(), to_cmp));
    }

    #[test]
    fn get_entries_invalid_range() {
        let mut journal = Journal::default();

        let yesterday = today().checked_sub_days(Days::new(1)).unwrap();
        let now = today();
        let tomorrow = today().checked_add_days(Days::new(1)).unwrap();
        let day_after_tomorrow = today().checked_add_days(Days::new(2)).unwrap();

        let yesterday_entry = JournalEntry {
            date: yesterday.clone(),
            mood: Rating::Bad,
            note: String::from("Yesterday"),
        };
        let now_entry = JournalEntry {
            date: now.clone(),
            mood: Rating::Neutral,
            note: String::from("Today"),
        };
        let day_after_tomorrow_entry = JournalEntry {
            date: day_after_tomorrow.clone(),
            mood: Rating::Great,
            note: String::from("Day after tomorrow"),
        };

        // Add in "wrong order"
        journal.add_entry(day_after_tomorrow_entry.clone());
        journal.add_entry(now_entry.clone());
        journal.add_entry(yesterday_entry.clone());

        assert!(journal.get_entries(&tomorrow, &yesterday).is_err());
    }
}
