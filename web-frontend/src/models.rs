use crate::client::QueueItem;

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Queue {
    items: Vec<QueueItem>,
    pub(crate) current: usize,
}

impl Queue {
    pub(crate) fn new(items: Vec<QueueItem>, current: Option<usize>) -> Self {
        Self {
            items,
            current: current.unwrap_or(0),
        }
    }

    pub(crate) fn current_item(&self) -> Option<&QueueItem> {
        self.items.get(self.current)
    }

    pub(crate) fn visible_items(&self) -> Option<&[QueueItem]> {
        if self.items.is_empty() {
            return None;
        }

        let range = self.current..self.items.len();
        Some(&self.items[range])
    }

    pub(crate) fn past_items(&self) -> Option<&[QueueItem]> {
        if self.items.is_empty() {
            return None;
        }
        if self.current == 0 {
            return None;
        }

        let range = 0..self.current;
        Some(&self.items[range])
    }

    pub(crate) fn can_move_to_next(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.current < self.items.len() - 1
    }

    pub(crate) fn can_move_to_previous(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.current != 0
    }

    pub(crate) fn can_play(&self) -> bool {
        self.has_visible_items()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn has_visible_items(&self) -> bool {
        !self.is_empty()
    }
}
