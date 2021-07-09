use std::fmt;

use rand::distributions::{Distribution, Standard};
use rand::Rng;

use yew::{html, Html};

use crate::util::Bounds;
use crate::{CardSinks, CardSources, CARD_HEIGHT, CARD_WIDTH};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SuitColour {
    Red,
    Black,
}

impl Suit {
    pub fn colour(&self) -> SuitColour {
        match self {
            Self::Spades | Self::Clubs => SuitColour::Black,
            Self::Diamonds | Self::Hearts => SuitColour::Red,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Clubs => "\u{2663}",
                Suit::Diamonds => "\u{2666}",
                Suit::Hearts => "\u{2665}",
                Suit::Spades => "\u{2660}",
            }
        )
    }
}

impl Distribution<Suit> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        match rng.gen_range(0..3) {
            0 => Suit::Hearts,
            1 => Suit::Spades,
            2 => Suit::Diamonds,
            _ => Suit::Clubs,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Value {
    pub fn next_value(&self) -> Option<Value> {
        match self {
            Value::Ace => Some(Value::Two),
            Value::Two => Some(Value::Three),
            Value::Three => Some(Value::Four),
            Value::Four => Some(Value::Five),
            Value::Five => Some(Value::Six),
            Value::Six => Some(Value::Seven),
            Value::Seven => Some(Value::Eight),
            Value::Eight => Some(Value::Nine),
            Value::Nine => Some(Value::Ten),
            Value::Ten => Some(Value::Jack),
            Value::Jack => Some(Value::Queen),
            Value::Queen => Some(Value::King),
            Value::King => None,
        }
    }

    pub fn prev_value(&self) -> Option<Value> {
        match self {
            Value::Ace => None,
            Value::Two => Some(Value::Ace),
            Value::Three => Some(Value::Two),
            Value::Four => Some(Value::Three),
            Value::Five => Some(Value::Four),
            Value::Six => Some(Value::Five),
            Value::Seven => Some(Value::Six),
            Value::Eight => Some(Value::Seven),
            Value::Nine => Some(Value::Eight),
            Value::Ten => Some(Value::Nine),
            Value::Jack => Some(Value::Ten),
            Value::Queen => Some(Value::Jack),
            Value::King => Some(Value::Queen),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Ace => "A",
                Value::Two => "2",
                Value::Three => "3",
                Value::Four => "4",
                Value::Five => "5",
                Value::Six => "6",
                Value::Seven => "7",
                Value::Eight => "8",
                Value::Nine => "9",
                Value::Ten => "10",
                Value::Jack => "J",
                Value::Queen => "Q",
                Value::King => "K",
            }
        )
    }
}

impl Distribution<Value> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Value {
        match rng.gen_range(0..12) {
            0 => Value::Ace,
            1 => Value::Two,
            2 => Value::Three,
            3 => Value::Four,
            4 => Value::Five,
            5 => Value::Six,
            6 => Value::Seven,
            7 => Value::Eight,
            8 => Value::Nine,
            9 => Value::Ten,
            10 => Value::Jack,
            11 => Value::Queen,
            _ => Value::King,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Card(pub Value, pub Suit);

pub const DECK: &[Card] = &[
    Card(Value::Ace, Suit::Spades),
    Card(Value::Two, Suit::Spades),
    Card(Value::Three, Suit::Spades),
    Card(Value::Four, Suit::Spades),
    Card(Value::Five, Suit::Spades),
    Card(Value::Six, Suit::Spades),
    Card(Value::Seven, Suit::Spades),
    Card(Value::Eight, Suit::Spades),
    Card(Value::Nine, Suit::Spades),
    Card(Value::Ten, Suit::Spades),
    Card(Value::Jack, Suit::Spades),
    Card(Value::Queen, Suit::Spades),
    Card(Value::King, Suit::Spades),
    Card(Value::Ace, Suit::Clubs),
    Card(Value::Two, Suit::Clubs),
    Card(Value::Three, Suit::Clubs),
    Card(Value::Four, Suit::Clubs),
    Card(Value::Five, Suit::Clubs),
    Card(Value::Six, Suit::Clubs),
    Card(Value::Seven, Suit::Clubs),
    Card(Value::Eight, Suit::Clubs),
    Card(Value::Nine, Suit::Clubs),
    Card(Value::Ten, Suit::Clubs),
    Card(Value::Jack, Suit::Clubs),
    Card(Value::Queen, Suit::Clubs),
    Card(Value::King, Suit::Clubs),
    Card(Value::Ace, Suit::Hearts),
    Card(Value::Two, Suit::Hearts),
    Card(Value::Three, Suit::Hearts),
    Card(Value::Four, Suit::Hearts),
    Card(Value::Five, Suit::Hearts),
    Card(Value::Six, Suit::Hearts),
    Card(Value::Seven, Suit::Hearts),
    Card(Value::Eight, Suit::Hearts),
    Card(Value::Nine, Suit::Hearts),
    Card(Value::Ten, Suit::Hearts),
    Card(Value::Jack, Suit::Hearts),
    Card(Value::Queen, Suit::Hearts),
    Card(Value::King, Suit::Hearts),
    Card(Value::Ace, Suit::Diamonds),
    Card(Value::Two, Suit::Diamonds),
    Card(Value::Three, Suit::Diamonds),
    Card(Value::Four, Suit::Diamonds),
    Card(Value::Five, Suit::Diamonds),
    Card(Value::Six, Suit::Diamonds),
    Card(Value::Seven, Suit::Diamonds),
    Card(Value::Eight, Suit::Diamonds),
    Card(Value::Nine, Suit::Diamonds),
    Card(Value::Ten, Suit::Diamonds),
    Card(Value::Jack, Suit::Diamonds),
    Card(Value::Queen, Suit::Diamonds),
    Card(Value::King, Suit::Diamonds),
];

pub enum CardVisual {
    Card(Card),
    Flipped,
    EmptySlot,
    Invisible,
}

impl CardVisual {
    pub fn as_html_from(&self, from_x: i32, from_y: i32, x: i32, y: i32) -> Html {
        self.as_html_custom(x, y, Some((from_x, from_y)), None)
    }

    fn as_html_custom_style(&self, custom_style: String) -> Html {
        match self {
            Self::Flipped => {
                html! { <div class="card flipped-card" style={ custom_style }></div> }
            }
            Self::EmptySlot => {
                html! { <div class="card empty-slot" style={ custom_style }></div>}
            }
            Self::Card(card) => {
                let Card(value, suit) = card;

                let card_class = match suit {
                    Suit::Clubs | Suit::Spades => "card card-base card-black",
                    Suit::Hearts | Suit::Diamonds => "card card-base card-red",
                };

                html! {
                    <div class={{card_class}} style={ custom_style }>
                        <span class="card-logo">{ value }<br/>{ suit }</span>
                        {{ Self::picture_html(card) }}
                        <span class="card-logo card-logo-flipped">{ value }<br/>{ suit }</span>
                    </div>
                }
            }
            Self::Invisible => html! {},
        }
    }

    pub fn as_html(&self, x: i32, y: i32) -> Html {
        self.as_html_custom(x, y, None, None)
    }
    pub fn as_draggable_html(&self, x: i32, y: i32) -> Html {
        self.as_html_custom(x, y, None, "cursor:move;")
    }
    pub fn as_draggable_html_from(&self, from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> Html {
        self.as_html_custom(to_x, to_y, Some((from_x, from_y)), "cursor:move;")
    }

    fn as_html_custom<'a, OS: Into<Option<&'a str>>>(
        &self,
        x: i32,
        y: i32,
        from: Option<(i32, i32)>,
        custom_style: OS,
    ) -> Html {
        let position_part = format!("left:{}px;top:{}px;", x, y);
        let animation_part = from.map_or(String::new(), |(sx, sy)| {
            format!(
                "--start-left:{}px;--start-top:{}px;animation:movingCard 0.2s linear 0s 1 forwards;",
                sx, sy
            )
        });
        let custom_part = custom_style.into().unwrap_or("");
        self.as_html_custom_style(format!(
            "{}{}{}",
            position_part, animation_part, custom_part
        ))
    }

    fn picture_html(card: &Card) -> Html {
        let Card(value, suit) = card;
        let e = &suit.to_string();
        match value {
            Value::Ace => html! {
                <span class="card-pic-single-letter">{{e}}</span>
            },
            Value::Two => html! {
                <span class="card-pic-number">{{e}}<br/>{{e}}</span>
            },
            Value::Three => html! {
                <span class="card-pic-number">
                    {{e}}<br/>
                    {{e}}<br/>
                    {{e}}
                </span>
            },
            Value::Four => html! {
                <span class="card-pic-number">
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}
                </span>
            },
            Value::Five => html! {
                <span class="card-pic-number">
                    {{e}}{{e}}<br/>
                    {{e}}<br/>
                    {{e}}{{e}}
                </span>
            },
            Value::Six => html! {
                <span class="card-pic-number">
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}</span>
            },
            Value::Seven => html! {
                <span class="card-pic-number">
                    {{e}}{{e}}<br/>
                    {{e}}<br/>
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}
                </span>
            },
            Value::Eight => html! {
                <span class="card-pic-number">
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}</span>
            },
            Value::Nine => html! {
                <span class="card-pic-number">
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}<br/>
                    {{e}}<br/>
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}
                </span>
            },
            Value::Ten => html! {
                <span class="card-pic-number">
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}<br/>
                    {{e}}{{e}}
                </span>
            },
            _ => html! {
                <span class="card-pic-single-letter">{ value }</span>
            },
        }
    }
}

#[derive(Debug)]
pub struct PhysicalCard {
    x: i32,
    y: i32,
    visible: bool,
    flipped: bool,
    card: Card,
    prev_loc: Option<(i32, i32)>,
}

impl PhysicalCard {
    pub fn new(x: i32, y: i32, card: &Card) -> Self {
        Self {
            x,
            y,
            visible: true,
            flipped: false,
            card: *card,
            prev_loc: None,
        }
    }

    fn card_visual(&self) -> CardVisual {
        if !self.visible {
            CardVisual::Invisible
        } else if self.flipped {
            CardVisual::Flipped
        } else {
            CardVisual::Card(self.card)
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        self.flipped = flipped;
    }

    pub fn set_prev_loc(&mut self, x: i32, y: i32) {
        self.prev_loc = Some((x, y));
    }

    pub fn card(&self) -> Card {
        self.card
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.prev_loc = Some((self.x, self.y));
        self.set_position(x, y);
    }

    pub fn within_bounds(&self, x: i32, y: i32) -> bool {
        Bounds::new(self.x, self.y, CARD_WIDTH, CARD_HEIGHT).contains(x, y)
    }

    pub fn as_html(&self) -> Html {
        self.as_html_custom(false)
    }

    pub fn as_draggable_html(&self) -> Html {
        self.as_html_custom(true)
    }

    fn as_html_custom(&self, draggable: bool) -> Html {
        let mut custom_style = None;
        if draggable {
            custom_style = Some("cursor: move;");
        }

        self.card_visual()
            .as_html_custom(self.x, self.y, self.prev_loc, custom_style)
    }
}

pub trait CardSource {
    fn card_source(&self) -> CardSources;
    fn borrow_card(&self) -> Option<&PhysicalCard>;
    fn borrow_card_mut(&mut self) -> Option<&mut PhysicalCard>;

    fn take_card(&mut self) -> Option<PhysicalCard>;
    fn peek_card(&self) -> Option<Card> {
        self.borrow_card().map(PhysicalCard::card)
    }
    fn set_release_location(&mut self, x: i32, y: i32) -> Result<(), ()> {
        if let Some(physical_card) = self.borrow_card_mut() {
            physical_card.set_prev_loc(x, y);
            Ok(())
        } else {
            Err(())
        }
    }
    fn set_mouse_release_location(&mut self, x: i32, y: i32) -> Result<(), ()> {
        let x = x - CARD_WIDTH as i32 / 2;
        let y = y - CARD_HEIGHT as i32 / 2;
        self.set_release_location(x, y)
    }
}

pub trait CardSink {
    fn card_sink(&self) -> CardSinks;
    fn place_card(
        &mut self,
        mouse_x: i32,
        mouse_y: i32,
        physical_cards: Vec<PhysicalCard>,
    ) -> Result<(), ()>;
    fn within_bounds(&self, x: i32, y: i32) -> bool;
    fn is_placement_possible(&self, cards: Vec<Card>) -> bool;
}
