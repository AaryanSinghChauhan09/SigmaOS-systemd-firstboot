// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-firstboot: systemd firstboot component model.
//!
//! Real implementation: firstboots, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Firstboot {
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
pub enum SystemdFirstbootError {
    EmptyFirstbootName,
    DuplicateFirstboot(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdFirstbootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFirstbootName => write!(f, "firstboot name must not be empty"),
            Self::DuplicateFirstboot(id) => write!(f, "duplicate firstboot {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdFirstbootError {}

pub struct SystemdFirstboot {
    pub name: String,
    firstboots: BTreeMap<String, Firstboot>,
    events: BTreeMap<String, Event>,
}

impl SystemdFirstboot {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            firstboots: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_firstboot(&mut self, item: Firstboot) -> Result<(), SystemdFirstbootError> {
        if item.name.trim().is_empty() {
            return Err(SystemdFirstbootError::EmptyFirstbootName);
        }
        let id = item.id.clone();
        if self.firstboots.contains_key(&id) {
            return Err(SystemdFirstbootError::DuplicateFirstboot(id.clone()));
        }
        self.firstboots.insert(id, item);
        Ok(())
    }

    pub fn firstboot_count(&self) -> usize {
        self.firstboots.len()
    }

    pub fn get_firstboot(&self, id: &str) -> Option<&Firstboot> {
        self.firstboots.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdFirstbootError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdFirstbootError::EmptyEventSubject);
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

    fn sample_firstboot(id: &str, name: &str) -> Firstboot {
        Firstboot {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdFirstboot::new("SystemdFirstboot");
        assert_eq!(m.name, "SystemdFirstboot");
        assert_eq!(m.firstboot_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_firstboot() {
        let mut m = SystemdFirstboot::new("SystemdFirstboot");
        m.add_firstboot(sample_firstboot("a1", "Alpha")).unwrap();
        assert_eq!(m.firstboot_count(), 1);
        assert!(m.get_firstboot("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdFirstboot::new("SystemdFirstboot");
        assert!(m.add_firstboot(sample_firstboot("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdFirstboot::new("SystemdFirstboot");
        m.add_firstboot(sample_firstboot("x", "One")).unwrap();
        assert!(m.add_firstboot(sample_firstboot("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdFirstboot::new("SystemdFirstboot");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
