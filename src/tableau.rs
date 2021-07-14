use std::cmp::max;

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
        let len = self.cards.len();
        html! {
            <>
                { CardVisual::EmptySlot.as_html(self.x, self.y, String::new()) }
                { for self.cards.iter().enumerate().map(
                    |(i, c)| if i==len-1 {
                        c.as_draggable_html()
                    } else {
                        c.as_html()
                    })
                }
            </>
        }
    }

    fn faceup_cards(&self) -> Vec<&PhysicalCard> {
        let mut cards: Vec<&PhysicalCard> = Vec::new();

        for card in self.cards.iter().rev() {
            if card.flipped() {
                break;
            }
            cards.push(card);
        }

        cards.reverse();
        cards
    }

    fn faceup_cards_mut(&mut self) -> Vec<&mut PhysicalCard> {
        let mut cards: Vec<&mut PhysicalCard> = Vec::new();

        for card in self.cards.iter_mut().rev() {
            if card.flipped() {
                break;
            }
            cards.push(card);
        }

        cards.reverse();
        cards
    }
}

impl CardSource for Tableau {
    fn card_source(&self) -> CardSources {
        self.source
    }

    fn borrow_cards(&self, count: usize) -> Vec<&PhysicalCard> {
        self.faceup_cards()
            .into_iter()
            .rev()
            .take(count)
            .rev()
            .collect()
    }

    fn borrow_cards_mut(&mut self, count: usize) -> Vec<&mut PhysicalCard> {
        self.faceup_cards_mut()
            .into_iter()
            .rev()
            .take(count)
            .rev()
            .collect()
    }

    fn take_cards(&mut self, num: usize) -> Vec<PhysicalCard> {
        let mut cards: Vec<PhysicalCard> = Vec::new();

        for _ in 0..num {
            if let Some(card) = self.cards.pop() {
                if card.flipped() {
                    self.cards.push(card);
                    break;
                }

                cards.push(card);
            } else {
                break;
            }
        }
        if let Some(new_top_card) = self.cards.last_mut() {
            new_top_card.set_flipped(false);
        }
        cards.reverse();
        cards
    }

    fn how_many_cards(&self, mouse_x: i32, mouse_y: i32) -> usize {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .find(|(_, card)| card.within_bounds(mouse_x, mouse_y) && !card.flipped())
            .map_or(0, |(i, _)| i + 1)
    }
}

impl CardSink for Tableau {
    fn card_sink(&self) -> CardSinks {
        self.sink
    }

    fn place_cards(
        &mut self,
        mouse_x: i32,
        mouse_y: i32,
        physical_cards: Vec<PhysicalCard>,
    ) -> Result<(), ()> {
        if self.is_placement_possible(&physical_cards.iter().map(PhysicalCard::card).collect()) {
            // Placement is only possible if there is one card

            for mut physical_card in physical_cards {
                let card_pos = self.cards.len() as i32;
                physical_card.set_xy(self.x, self.y + STACKED_CARD_Y_STRIDE * card_pos);
                physical_card.set_prev_loc(
                    mouse_x - CARD_WIDTH as i32 / 2,
                    mouse_y - CARD_HEIGHT as i32 / 2,
                );
                physical_card.set_visible(true);
                self.cards.push(physical_card);
            }

            Ok(())
        } else {
            Err(())
        }
    }

    fn within_bounds(&self, x: i32, y: i32) -> bool {
        Bounds::new(
            self.x,
            self.y + STACKED_CARD_Y_STRIDE * max(self.cards.len() as i32 - 1, 0),
            CARD_WIDTH,
            CARD_HEIGHT,
        )
        .contains(x, y)
    }

    fn is_placement_possible(&self, cards: &Vec<Card>) -> bool {
        let Card(value, suit) = *cards.first().expect("card should be present");
        if let Some(Card(top_value, top_suit)) = self.cards.last().map(PhysicalCard::card) {
            top_value.prev_value().map_or(false, |v| v == value)
                && top_suit.colour() != suit.colour()
        } else {
            value == Value::King
        }
    }
}
