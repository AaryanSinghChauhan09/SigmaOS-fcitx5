// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-fcitx5: fcitx5 component model.
//!
//! Real implementation: fcitxs, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fcitx {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fcitx5Error {
    EmptyFcitxName,
    DuplicateFcitx(String),
    EmptyEventSubject,
}

impl fmt::Display for Fcitx5Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFcitxName => write!(f, "fcitx name must not be empty"),
            Self::DuplicateFcitx(id) => write!(f, "duplicate fcitx {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for Fcitx5Error {}

pub struct Fcitx5 {
    pub name: String,
    fcitxs: BTreeMap<String, Fcitx>,
    events: BTreeMap<String, Event>,
}

impl Fcitx5 {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fcitxs: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_fcitx(&mut self, item: Fcitx) -> Result<(), Fcitx5Error> {
        if item.name.trim().is_empty() {
            return Err(Fcitx5Error::EmptyFcitxName);
        }
        let id = item.id.clone();
        if self.fcitxs.contains_key(&id) {
            return Err(Fcitx5Error::DuplicateFcitx(id.clone()));
        }
        self.fcitxs.insert(id, item);
        Ok(())
    }

    pub fn fcitx_count(&self) -> usize {
        self.fcitxs.len()
    }

    pub fn get_fcitx(&self, id: &str) -> Option<&Fcitx> {
        self.fcitxs.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), Fcitx5Error> {
        if event.subject.trim().is_empty() {
            return Err(Fcitx5Error::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_fcitx(id: &str, name: &str) -> Fcitx {
        Fcitx {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Fcitx5::new("Fcitx5");
        assert_eq!(m.name, "Fcitx5");
        assert_eq!(m.fcitx_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_fcitx() {
        let mut m = Fcitx5::new("Fcitx5");
        m.add_fcitx(sample_fcitx("a1", "Alpha")).unwrap();
        assert_eq!(m.fcitx_count(), 1);
        assert!(m.get_fcitx("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Fcitx5::new("Fcitx5");
        assert!(m.add_fcitx(sample_fcitx("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Fcitx5::new("Fcitx5");
        m.add_fcitx(sample_fcitx("x", "One")).unwrap();
        assert!(m.add_fcitx(sample_fcitx("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Fcitx5::new("Fcitx5");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
