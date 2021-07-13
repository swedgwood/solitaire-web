use yew::{html, Html};

use crate::{
    card::{Card, CardSink, CardSource, CardVisual, PhysicalCard, Value},
    util::Bounds,
    CardSinks, CardSources, CARD_HEIGHT, CARD_WIDTH,
};

pub struct Foundation {
    sink: CardSinks,
    source: CardSources,
    cards: Vec<PhysicalCard>,
    x: i32,
    y: i32,
}

impl Foundation {
    pub fn new(x: i32, y: i32, sink: CardSinks, source: CardSources) -> Self {
        Self {
            sink,
            source,
            cards: Vec::new(),
            x,
            y,
        }
    }

    pub fn as_html(&self) -> Html {
        if let Some(top_card) = self.cards.last() {
            html! {
                { top_card.as_draggable_html() }
            }
        } else {
            html! {
                { CardVisual::EmptySlot.as_html(self.x, self.y) }
            }
        }
    }
}

impl CardSink for Foundation {
    fn card_sink(&self) -> CardSinks {
        self.sink
    }

    fn is_placement_possible(&self, cards: &Vec<Card>) -> bool {
        if cards.len() == 1 {
            let card = *cards.first().expect("card should be present");
            let Card(value, suit) = card;

            if let Some(top_card) = self.cards.last() {
                let Card(top_value, top_suit) = top_card.card();

                if let Some(next_value) = top_value.next_value() {
                    top_suit == suit && next_value == value
                } else {
                    false
                }
            } else {
                value == Value::Ace
            }
        } else {
            false
        }
    }

    // Returns an Err is placement is not possible
    fn place_cards(
        &mut self,
        mouse_x: i32,
        mouse_y: i32,
        mut physical_cards: Vec<PhysicalCard>,
    ) -> Result<(), ()> {
        if self.is_placement_possible(&physical_cards.iter().map(PhysicalCard::card).collect()) {
            // Placement is only possible if there is one card
            let mut physical_card = physical_cards.pop().expect("card should be present");
            physical_card.set_position(self.x, self.y);
            physical_card.set_prev_loc(
                mouse_x - CARD_WIDTH as i32 / 2,
                mouse_y - CARD_HEIGHT as i32 / 2,
            );
            physical_card.set_visible(true);
            self.cards.push(physical_card);
            Ok(())
        } else {
            Err(())
        }
    }

    fn within_bounds(&self, x: i32, y: i32) -> bool {
        Bounds::new(self.x, self.y, CARD_WIDTH, CARD_HEIGHT).contains(x, y)
    }
}

impl CardSource for Foundation {
    fn take_cards(&mut self, num: usize) -> Vec<PhysicalCard> {
        if num > 0 {
            self.cards.pop().map_or_else(Vec::new, |c| vec![c])
        } else {
            Vec::new()
        }
    }

    fn borrow_cards(&self, count: usize) -> Vec<&PhysicalCard> {
        if count > 0 {
            self.cards.last().map_or_else(Vec::new, |c| vec![c])
        } else {
            Vec::new()
        }
    }

    fn card_source(&self) -> CardSources {
        self.source
    }

    fn borrow_cards_mut(&mut self, count: usize) -> Vec<&mut PhysicalCard> {
        if count > 0 {
            self.cards.last_mut().map_or_else(Vec::new, |c| vec![c])
        } else {
            Vec::new()
        }
    }

    fn how_many_cards(&self, mouse_x: i32, mouse_y: i32) -> usize {
        if let Some(physical_card) = self.cards.last() {
            if physical_card.within_bounds(mouse_x, mouse_y) {
                1
            } else {
                0
            }
        } else {
            0
        }
    }
}
