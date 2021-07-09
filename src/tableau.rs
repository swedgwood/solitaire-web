use yew::{html, Html};

use crate::{
    card::{Card, CardSink, CardSource, CardVisual, PhysicalCard, Value},
    util::Bounds,
    CardSinks, CardSources, CARD_HEIGHT, CARD_WIDTH, STACKED_CARD_Y_STRIDE,
};

pub struct Tableau {
    cards: Vec<PhysicalCard>,
    sink: CardSinks,
    source: CardSources,
    x: i32,
    y: i32,
}

impl Tableau {
    pub fn new(x: i32, y: i32, sink: CardSinks, source: CardSources) -> Self {
        Self {
            cards: Vec::new(),
            sink,
            source,
            x,
            y,
        }
    }

    pub fn from_cards(
        x: i32,
        y: i32,
        cards: Vec<PhysicalCard>,
        sink: CardSinks,
        source: CardSources,
    ) -> Self {
        let mut tableau = Tableau::new(x, y, sink, source);
        tableau.cards = cards;
        let len = tableau.cards.len();
        tableau.cards.iter_mut().enumerate().for_each(|(i, card)| {
            card.set_position(x, y + STACKED_CARD_Y_STRIDE * i as i32);
            if i == len - 1 {
                card.set_flipped(false);
            } else {
                card.set_flipped(true);
            }
        });
        tableau
    }

    pub fn as_html(&self) -> Html {
        html! {
            <>
            { CardVisual::EmptySlot.as_html(self.x, self.y) }
            { for self.cards.iter().map(|c| c.as_html() ) }
            </>
        }
    }
}

impl CardSource for Tableau {
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
        let card = self.cards.pop();
        if let Some(new_top_card) = self.cards.last_mut() {
            new_top_card.set_flipped(false);
        }
        card
    }
}

impl CardSink for Tableau {
    fn card_sink(&self) -> CardSinks {
        self.sink
    }

    fn place_card(
        &mut self,
        mouse_x: i32,
        mouse_y: i32,
        mut physical_card: PhysicalCard,
    ) -> Result<(), ()> {
        if self.is_placement_possible(physical_card.card()) {
            let card_pos = self.cards.len() as i32;
            physical_card.set_position(self.x, self.y + STACKED_CARD_Y_STRIDE * card_pos);
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
        Bounds::new(
            self.x,
            self.y + 30 * self.cards.len() as i32,
            CARD_WIDTH,
            CARD_HEIGHT,
        )
        .contains(x, y)
    }

    fn is_placement_possible(&self, card: Card) -> bool {
        let Card(value, suit) = card;
        if let Some(Card(top_value, top_suit)) = self.cards.last().map(PhysicalCard::card) {
            top_value.prev_value().map_or(false, |v| v == value)
                && top_suit.colour() != suit.colour()
        } else {
            value == Value::King
        }
    }
}
