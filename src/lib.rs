// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-system-monitor: system monitor component model.
//!
//! Real implementation: monitors, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Monitor {
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
pub enum SystemMonitorError {
    EmptyMonitorName,
    DuplicateMonitor(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemMonitorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMonitorName => write!(f, "monitor name must not be empty"),
            Self::DuplicateMonitor(id) => write!(f, "duplicate monitor {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemMonitorError {}

pub struct SystemMonitor {
    pub name: String,
    monitors: BTreeMap<String, Monitor>,
    events: BTreeMap<String, Event>,
}

impl SystemMonitor {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            monitors: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_monitor(&mut self, item: Monitor) -> Result<(), SystemMonitorError> {
        if item.name.trim().is_empty() {
            return Err(SystemMonitorError::EmptyMonitorName);
        }
        let id = item.id.clone();
        if self.monitors.contains_key(&id) {
            return Err(SystemMonitorError::DuplicateMonitor(id.clone()));
        }
        self.monitors.insert(id, item);
        Ok(())
    }

    pub fn monitor_count(&self) -> usize {
        self.monitors.len()
    }

    pub fn get_monitor(&self, id: &str) -> Option<&Monitor> {
        self.monitors.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemMonitorError> {
        if event.subject.trim().is_empty() {
            return Err(SystemMonitorError::EmptyEventSubject);
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

    fn sample_monitor(id: &str, name: &str) -> Monitor {
        Monitor {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemMonitor::new("SystemMonitor");
        assert_eq!(m.name, "SystemMonitor");
        assert_eq!(m.monitor_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_monitor() {
        let mut m = SystemMonitor::new("SystemMonitor");
        m.add_monitor(sample_monitor("a1", "Alpha")).unwrap();
        assert_eq!(m.monitor_count(), 1);
        assert!(m.get_monitor("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemMonitor::new("SystemMonitor");
        assert!(m.add_monitor(sample_monitor("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemMonitor::new("SystemMonitor");
        m.add_monitor(sample_monitor("x", "One")).unwrap();
        assert!(m.add_monitor(sample_monitor("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemMonitor::new("SystemMonitor");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
