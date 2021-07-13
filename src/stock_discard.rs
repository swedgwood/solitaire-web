use std::cmp::max;

use yew::{html, Html};

use crate::{
    card::{CardSource, CardVisual, PhysicalCard},
    util::Bounds,
    CardSources, CARD_HEIGHT, CARD_WIDTH, CARD_X_STRIDE, STACKED_CARD_X_STRIDE,
};

pub struct Stock {
    cards: Vec<PhysicalCard>,
    bounds: Bounds,
    x: i32,
    y: i32,
}

impl Stock {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            cards: Vec::new(),
            bounds: Bounds::new(x, y, CARD_WIDTH, CARD_HEIGHT),
            x,
            y,
        }
    }

    pub fn from_cards(x: i32, y: i32, mut cards: Vec<PhysicalCard>) -> Self {
        let mut stock = Self::new(x, y);
        cards.iter_mut().for_each(|card| {
            card.set_position(stock.x, stock.y);
            card.set_flipped(true)
        });
        stock.cards = cards;
        stock
    }

    pub fn as_html(&self) -> Html {
        let card_html = match self.cards.last() {
            Some(card) => card.as_html(),
            None => html! {},
        };

        html! {
            <>
                { CardVisual::EmptySlot.as_html(self.x, self.y, String::new()) }
                { card_html }
            </>
        }
    }

    fn take_3_cards(&mut self) -> Vec<PhysicalCard> {
        let mut cards: Vec<PhysicalCard> = Vec::new();

        for _ in 0..3 {
            if let Some(card) = self.cards.pop() {
                cards.push(card);
            }
        }

        cards
    }

    fn deposit_cards(&mut self, mut cards: Vec<PhysicalCard>) {
        let (x, y) = (self.x, self.y);
        let last_index = max(0, cards.len() as i32 - 1) as usize;
        cards.iter_mut().enumerate().for_each(|(i, c)| {
            if i == last_index {
                c.move_to(x, y)
            } else {
                c.set_position(x, y);
            }

            c.set_flipped(true)
        });
        self.cards.append(&mut cards);
    }

    fn within_bounds(&self, x: i32, y: i32) -> bool {
        self.bounds.contains(x, y)
    }
}

pub struct Discard {
    cards: Vec<PhysicalCard>,
    x: i32,
    y: i32,
}

impl Discard {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            cards: Vec::new(),
            x,
            y,
        }
    }

    fn add_3_cards(&mut self, mut cards: Vec<PhysicalCard>) {
        self.cards.append(&mut cards);

        let len = self.cards.len();
        let self_x = self.x;
        let self_y = self.y;

        self.cards
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| len - 1 - i <= 5)
            .for_each(|(index, card)| {
                let mut x = self_x;

                if len - index <= 2 {
                    let offset = 3 - (len - index) as i32; // Should be a value in {1, 2}
                    x += offset * STACKED_CARD_X_STRIDE;
                }

                card.move_to(x, self_y);
                card.set_flipped(false);
            });
    }

    fn take_cards(&mut self) -> Vec<PhysicalCard> {
        let mut cards = Vec::new();
        std::mem::swap(&mut self.cards, &mut cards);
        cards
    }

    pub fn as_html(&self) -> Html {
        let end = self.cards.len();
        let start = max(end as i32 - 6, 0) as usize;
        html! {
            <>
                { CardVisual::EmptySlot.as_html(self.x, self.y, String::new()) }
                { for (start..end).map(|i| if i==end {
                    self.cards[i].as_draggable_html()
                } else {
                    self.cards[i].as_html()
                }) }
            </>
        }
    }
}

impl CardSource for Discard {
    fn take_cards(&mut self, num: usize) -> Vec<PhysicalCard> {
        if num > 0 {
            let return_value = self.cards.pop().map_or_else(Vec::new, |c| vec![c]);
            let len = self.cards.len();
            let (x, y) = (self.x, self.y);
            self.cards
                .iter_mut()
                .enumerate()
                .map(|(i, c)| ((len - i - 1) as i32, c))
                .filter(|(i, _)| *i < 3)
                .for_each(|(i, c)| c.move_to(x + (2 - i) * STACKED_CARD_X_STRIDE, y));

            return_value
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
        CardSources::Discard
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

pub struct StockDiscard {
    stock: Stock,
    discard: Discard,
}

impl StockDiscard {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            stock: Stock::new(x, y),
            discard: Discard::new(x + CARD_X_STRIDE, y),
        }
    }

    pub fn from_cards(x: i32, y: i32, cards: Vec<PhysicalCard>) -> Self {
        Self {
            stock: Stock::from_cards(x, y, cards),
            discard: Discard::new(x + CARD_X_STRIDE, y),
        }
    }

    pub fn discard_mut(&mut self) -> &mut Discard {
        &mut self.discard
    }

    pub fn handle_click(&mut self, x: i32, y: i32) -> bool {
        if self.stock.within_bounds(x, y) {
            self.deal_into_discard();
            true
        } else {
            false
        }
    }

    pub fn deal_into_discard(&mut self) {
        let cards = self.stock.take_3_cards();

        if cards.is_empty() {
            let mut cards = self.discard.take_cards();
            cards.reverse();
            self.stock.deposit_cards(cards);
        } else {
            self.discard.add_3_cards(cards);
        }
    }

    pub fn as_html(&self) -> Html {
        html! {
            <>
            { self.stock.as_html() }
            { self.discard.as_html() }
            </>
        }
    }
}
