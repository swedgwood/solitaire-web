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
        html! {
            <>
            { CardVisual::EmptySlot.as_html(self.x, self.y) }
            { for self.cards.iter().map(PhysicalCard::as_html) }
            </>
        }
    }
}

impl CardSink for Foundation {
    fn card_sink(&self) -> CardSinks {
        self.sink
    }

    fn is_placement_possible(&self, cards: Vec<Card>) -> bool {
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
    fn place_card(
        &mut self,
        mouse_x: i32,
        mouse_y: i32,
        mut physical_cards: Vec<PhysicalCard>,
    ) -> Result<(), ()> {
        if self.is_placement_possible(physical_cards.iter().map(PhysicalCard::card).collect()) {
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
    fn card_source(&self) -> CardSources {
        self.source
    }

    fn borrow_card(&self) -> Option<&PhysicalCard> {
        self.cards.last()
    }

    fn borrow_card_mut(&mut self) -> Option<&mut PhysicalCard> {
        self.cards.last_mut()
    }

    fn take_card(&mut self) -> Option<PhysicalCard> {
        self.cards.pop()
    }
}
