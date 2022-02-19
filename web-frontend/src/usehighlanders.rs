use crate::prelude::*;
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

#[derive(Clone, Debug, Default)]
pub(crate) struct Highlanders {
    members: HashMap<String, bool>,
    children: Vec<HighlandersChild>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct HighlandersChild(Vec<String>);

pub(crate) struct UseHighlanders {
    update_callback: Rc<dyn Fn()>,
    pub(crate) value: Rc<RefCell<Highlanders>>,
}

pub(crate) fn use_highlanders<'src>(cx: &'src Scope) -> &'src mut UseHighlanders {
    cx.use_hook(|_| UseHighlanders {
        update_callback: cx.schedule_update(),
        value: Rc::new(RefCell::new(Highlanders::default())),
    })
}

impl UseHighlanders {
    pub(crate) fn enable(&self, key: &str) -> bool {
        self.disable_all();
        if let Some(v) = self.value.borrow_mut().members.get_mut(key) {
            *v = true;
            (self.update_callback)();
            return true;
        }
        return false;
    }

    pub(crate) fn is_enabled(&self, key: &str) -> bool {
        if let Some(v) = self.value.borrow().members.get(key) {
            return *v;
        }
        false
    }

    pub(crate) fn disable_all(&self) {
        for v in self.value.borrow_mut().members.values_mut() {
            *v = false;
        }
        (self.update_callback)();
    }

    pub(crate) fn register(&self, key: &str) {
        let mut highlanders = self.value.borrow_mut();
        let last_idx = highlanders.children.len() - 1;
        let c = highlanders.children.get_mut(last_idx).unwrap();
        c.0.push(key.to_string());
        highlanders.members.insert(key.to_string(), false);
    }
}

impl Clone for UseHighlanders {
    fn clone(&self) -> Self {
        log::info!("clone {:?}", self);
        self.value
            .borrow_mut()
            .children
            .push(HighlandersChild::default());
        Self {
            update_callback: self.update_callback.clone(),
            value: self.value.clone(),
        }
    }
}

impl Drop for UseHighlanders {
    fn drop(&mut self) {
        log::info!("drop {:?}", self);
        let mut highlanders = self.value.borrow_mut();
        // This should always be Some but in case something goes horribly
        // wrong it's probably better to silently fail than panic?
        if let Some(c) = highlanders.children.pop() {
            for key in c.0 {
                highlanders.members.remove(&key);
            }
        }
    }
}

impl Debug for UseHighlanders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value.borrow())
    }
}
