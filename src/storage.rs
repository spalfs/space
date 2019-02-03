use crate::item::Item;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage {
    items: Vec<Item>,
    carrying: usize,
    capacity: usize,
}

impl Storage {
    pub fn new(items: Vec<Item>) -> Storage {
        let mut carrying = 0;
        for item in items.iter() {
            carrying += item.size;
        }
        Storage {
            items,
            capacity: 10,
            carrying,
        }
    }

    pub fn has_minerals(&self) -> bool {
        self.items.iter().any(|item| item.is_mineral())
    }

    pub fn refined_count(&self) -> usize {
        let mut items = self.items.clone();
        items.retain(|item| item.name == "Refined Mineral");
        items.len()
    }

    pub fn take(&mut self, name: &str) -> Option<Item> {
        match self.items.iter().position(|item| item.name == name) {
            Some(index) => {
                let item = self.items.remove(index);
                self.carrying -= item.size;
                Some(item)
            }
            None => None,
        }
    }

    pub fn give(&mut self, item: Item) -> bool {
        if self.capacity >= self.carrying + item.size {
            self.carrying += item.size;
            self.items.push(item);
            true
        } else {
            false
        }
    }
}
