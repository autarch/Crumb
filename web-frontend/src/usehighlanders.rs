use std::collections::HashMap;

#[derive(Debug, Default)]
pub(crate) struct Highlanders {
    members: HashMap<String, bool>,
}

impl Highlanders {
    pub(crate) fn enable(&mut self, key: &str) -> bool {
        self.disable_all();
        if let Some(v) = self.members.get_mut(key) {
            *v = true;
            return true;
        }
        return false;
    }

    pub(crate) fn disable(&mut self, key: &str) -> bool {
        if let Some(v) = self.members.get_mut(key) {
            *v = false;
            return true;
        }
        return false;
    }

    pub(crate) fn is_enabled(&self, key: &str) -> bool {
        if let Some(v) = self.members.get(key) {
            return *v;
        }
        false
    }

    pub(crate) fn disable_all(&mut self) {
        for v in self.members.values_mut() {
            if *v {
                *v = false;
            }
        }
    }

    pub(crate) fn register(&mut self, key: &str) {
        self.members.entry(key.to_string()).or_insert_with(|| false);
    }
}
