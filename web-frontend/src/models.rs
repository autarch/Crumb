use crate::client::QueueItem;

#[derive(Debug, Default, PartialEq)]
pub struct Queue {
    items: Vec<QueueItem>,
    current: usize,
}

impl Queue {
    pub fn new(items: Vec<QueueItem>, current: Option<usize>) -> Self {
        Self {
            items,
            current: current.unwrap_or(0),
        }
    }

    pub fn current_item(&self) -> Option<&QueueItem> {
        self.items.get(self.current)
    }

    pub fn visible_items(&self) -> Option<impl Iterator<Item = &QueueItem>> {
        if self.items.is_empty() {
            return None;
        }

        let range = self.current..self.items.len();
        Some(self.items[range].iter())
    }

    pub fn can_move_to_next(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.current < self.items.len() - 1
    }

    pub fn can_move_to_previous(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.current != 0
    }

    pub fn can_play(&self) -> bool {
        self.has_visible_items()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn has_visible_items(&self) -> bool {
        !self.is_empty()
    }
}
