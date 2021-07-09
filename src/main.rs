use foundation::Foundation;
use rand::{prelude::SliceRandom, thread_rng};
use tableau::Tableau;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Document, MouseEvent};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod card;
mod foundation;
mod stock_discard;
mod tableau;
mod util;

use card::{Card, CardSink, CardSource, CardVisual, PhysicalCard, DECK};
use stock_discard::StockDiscard;

const CARD_WIDTH: u32 = 125;
const CARD_HEIGHT: u32 = 175;

const PADDING: i32 = 10;

const STACKED_CARD_X_STRIDE: i32 = 35;
const STACKED_CARD_Y_STRIDE: i32 = 45;

const CARD_X_STRIDE: i32 = CARD_WIDTH as i32 + 20;
const CARD_Y_STRIDE: i32 = CARD_HEIGHT as i32 + 20;

const STOCK_DISCARD_X: i32 = PADDING;
const STOCK_DISCARD_Y: i32 = PADDING;

const FOUNDATIONS_X: i32 = PADDING + 3 * CARD_X_STRIDE;
const FOUNDATIONS_Y: i32 = PADDING;

const TABLEAUS_Y: i32 = PADDING + CARD_Y_STRIDE;
const TABLEAUS_X: i32 = PADDING;

#[allow(clippy::enum_variant_names)]
enum Msg {
    MouseUp(i32, i32),
    MouseDown(i32, i32),
    // x, y, dx, dy
    MouseMove(i32, i32, i32, i32),
}

#[derive(Clone, Copy, Debug)]
pub enum CardSources {
    Discard,
    Foundation1,
    Foundation2,
    Foundation3,
    Foundation4,
    Tableau1,
    Tableau2,
    Tableau3,
    Tableau4,
    Tableau5,
    Tableau6,
    Tableau7,
}

#[derive(Clone, Copy, Debug)]
pub enum CardSinks {
    Foundation1,
    Foundation2,
    Foundation3,
    Foundation4,
    Tableau1,
    Tableau2,
    Tableau3,
    Tableau4,
    Tableau5,
    Tableau6,
    Tableau7,
}

#[derive(Clone)]
struct HeldCard {
    card: Vec<Card>,
    source: CardSources,
    x: i32,
    y: i32,
    prev_pos: Option<(i32, i32)>,
}

impl HeldCard {
    pub fn new(source: &dyn CardSource, mouse_x: i32, mouse_y: i32) -> Self {
        let card = source
            .peek_card()
            .expect("By this point, card should be available");
        let card_source = source.card_source();
        let (x, y) = source
            .borrow_card()
            .expect("By this point, card should be available")
            .position();
        Self {
            card: vec![card],
            source: card_source,
            x: mouse_x - CARD_WIDTH as i32 / 2,
            y: mouse_y - CARD_HEIGHT as i32 / 2,
            prev_pos: Some((x, y)),
        }
    }
    pub fn source(&self) -> CardSources {
        self.source
    }

    pub fn set_mouse_position(&mut self, x: i32, y: i32) {
        self.set_position(x - CARD_WIDTH as i32 / 2, y - CARD_HEIGHT as i32 / 2);
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.prev_pos = Some((self.x, self.y));
        self.x = x;
        self.y = y;
    }

    pub fn as_html(&self) -> Html {
        if let Some((from_x, from_y)) = self.prev_pos {
            CardVisual::Card(self.card[0]).as_draggable_html_from(from_x, from_y, self.x, self.y)
        } else {
            CardVisual::Card(self.card[0]).as_draggable_html(self.x, self.y)
        }
    }
}

struct Model {
    stock_discard: StockDiscard,
    foundation1: Foundation,
    foundation2: Foundation,
    foundation3: Foundation,
    foundation4: Foundation,
    tableau1: Tableau,
    tableau2: Tableau,
    tableau3: Tableau,
    tableau4: Tableau,
    tableau5: Tableau,
    tableau6: Tableau,
    tableau7: Tableau,
    held_card: Option<HeldCard>,
}

impl Model {
    fn borrow_sources(&mut self) -> Vec<&mut dyn CardSource> {
        vec![
            self.stock_discard.discard_mut(),
            &mut self.foundation1,
            &mut self.foundation2,
            &mut self.foundation3,
            &mut self.foundation4,
            &mut self.tableau1,
            &mut self.tableau2,
            &mut self.tableau3,
            &mut self.tableau4,
            &mut self.tableau5,
            &mut self.tableau6,
            &mut self.tableau7,
        ]
    }

    fn borrow_sinks(&mut self) -> Vec<&mut dyn CardSink> {
        vec![
            &mut self.foundation1,
            &mut self.foundation2,
            &mut self.foundation3,
            &mut self.foundation4,
            &mut self.tableau1,
            &mut self.tableau2,
            &mut self.tableau3,
            &mut self.tableau4,
            &mut self.tableau5,
            &mut self.tableau6,
            &mut self.tableau7,
        ]
    }

    fn borrow_source(&mut self, source: CardSources) -> &mut dyn CardSource {
        match source {
            CardSources::Discard => self.stock_discard.discard_mut(),
            CardSources::Foundation1 => &mut self.foundation1,
            CardSources::Foundation2 => &mut self.foundation2,
            CardSources::Foundation3 => &mut self.foundation3,
            CardSources::Foundation4 => &mut self.foundation4,
            CardSources::Tableau1 => &mut self.tableau1,
            CardSources::Tableau2 => &mut self.tableau2,
            CardSources::Tableau3 => &mut self.tableau3,
            CardSources::Tableau4 => &mut self.tableau4,
            CardSources::Tableau5 => &mut self.tableau5,
            CardSources::Tableau6 => &mut self.tableau6,
            CardSources::Tableau7 => &mut self.tableau7,
        }
    }

    fn borrow_sink(&mut self, sink: CardSinks) -> &mut dyn CardSink {
        match sink {
            CardSinks::Foundation1 => &mut self.foundation1,
            CardSinks::Foundation2 => &mut self.foundation2,
            CardSinks::Foundation3 => &mut self.foundation3,
            CardSinks::Foundation4 => &mut self.foundation4,
            CardSinks::Tableau1 => &mut self.tableau1,
            CardSinks::Tableau2 => &mut self.tableau2,
            CardSinks::Tableau3 => &mut self.tableau3,
            CardSinks::Tableau4 => &mut self.tableau4,
            CardSinks::Tableau5 => &mut self.tableau5,
            CardSinks::Tableau6 => &mut self.tableau6,
            CardSinks::Tableau7 => &mut self.tableau7,
        }
    }

    fn borrow_held_source(&mut self, held_card: HeldCard) -> &mut dyn CardSource {
        self.borrow_source(held_card.source())
    }

    fn setup_event_callbacks(document: &Document, link: &ComponentLink<Self>) {
        let mouseup_callback = link.callback(|e: MouseEvent| Msg::MouseUp(e.page_x(), e.page_y()));
        let mouseup_closure =
            Closure::wrap(Box::new(move |e: MouseEvent| mouseup_callback.emit(e))
                as Box<dyn FnMut(MouseEvent)>);
        document.set_onmouseup(Some(mouseup_closure.as_ref().unchecked_ref()));
        mouseup_closure.forget();

        let mousedown_callback =
            link.callback(|e: MouseEvent| Msg::MouseDown(e.page_x(), e.page_y()));
        let mousedown_closure =
            Closure::wrap(Box::new(move |e: MouseEvent| mousedown_callback.emit(e))
                as Box<dyn FnMut(MouseEvent)>);
        document.set_onmousedown(Some(mousedown_closure.as_ref().unchecked_ref()));
        mousedown_closure.forget();

        let mousemove_callback = link.callback(|e: MouseEvent| {
            Msg::MouseMove(e.page_x(), e.page_y(), e.movement_x(), e.movement_y())
        });
        let mousemove_closure =
            Closure::wrap(Box::new(move |e: MouseEvent| mousemove_callback.emit(e))
                as Box<dyn FnMut(MouseEvent)>);
        document.set_onmousemove(Some(mousemove_closure.as_ref().unchecked_ref()));
        mousemove_closure.forget();
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut stock_cards: Vec<PhysicalCard> =
            DECK.iter().map(|c| PhysicalCard::new(0, 0, c)).collect();
        stock_cards.shuffle(&mut thread_rng());

        let tableau1_cards: Vec<PhysicalCard> = stock_cards.drain(0..1).collect();
        let tableau2_cards: Vec<PhysicalCard> = stock_cards.drain(0..2).collect();
        let tableau3_cards: Vec<PhysicalCard> = stock_cards.drain(0..3).collect();
        let tableau4_cards: Vec<PhysicalCard> = stock_cards.drain(0..4).collect();
        let tableau5_cards: Vec<PhysicalCard> = stock_cards.drain(0..5).collect();
        let tableau6_cards: Vec<PhysicalCard> = stock_cards.drain(0..6).collect();
        let tableau7_cards: Vec<PhysicalCard> = stock_cards.drain(0..7).collect();

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        Self::setup_event_callbacks(&document, &link);

        Self {
            stock_discard: StockDiscard::from_cards(STOCK_DISCARD_X, STOCK_DISCARD_Y, stock_cards),
            foundation1: Foundation::new(
                FOUNDATIONS_X,
                FOUNDATIONS_Y,
                CardSinks::Foundation1,
                CardSources::Foundation1,
            ),
            foundation2: Foundation::new(
                FOUNDATIONS_X + CARD_X_STRIDE,
                FOUNDATIONS_Y,
                CardSinks::Foundation2,
                CardSources::Foundation2,
            ),
            foundation3: Foundation::new(
                FOUNDATIONS_X + CARD_X_STRIDE * 2,
                FOUNDATIONS_Y,
                CardSinks::Foundation3,
                CardSources::Foundation3,
            ),
            foundation4: Foundation::new(
                FOUNDATIONS_X + CARD_X_STRIDE * 3,
                FOUNDATIONS_Y,
                CardSinks::Foundation4,
                CardSources::Foundation4,
            ),
            tableau1: Tableau::from_cards(
                TABLEAUS_X,
                TABLEAUS_Y,
                tableau1_cards,
                CardSinks::Tableau1,
                CardSources::Tableau1,
            ),
            tableau2: Tableau::from_cards(
                TABLEAUS_X + CARD_X_STRIDE,
                TABLEAUS_Y,
                tableau2_cards,
                CardSinks::Tableau2,
                CardSources::Tableau2,
            ),
            tableau3: Tableau::from_cards(
                TABLEAUS_X + CARD_X_STRIDE * 2,
                TABLEAUS_Y,
                tableau3_cards,
                CardSinks::Tableau3,
                CardSources::Tableau3,
            ),
            tableau4: Tableau::from_cards(
                TABLEAUS_X + CARD_X_STRIDE * 3,
                TABLEAUS_Y,
                tableau4_cards,
                CardSinks::Tableau4,
                CardSources::Tableau4,
            ),
            tableau5: Tableau::from_cards(
                TABLEAUS_X + CARD_X_STRIDE * 4,
                TABLEAUS_Y,
                tableau5_cards,
                CardSinks::Tableau5,
                CardSources::Tableau5,
            ),
            tableau6: Tableau::from_cards(
                TABLEAUS_X + CARD_X_STRIDE * 5,
                TABLEAUS_Y,
                tableau6_cards,
                CardSinks::Tableau6,
                CardSources::Tableau6,
            ),
            tableau7: Tableau::from_cards(
                TABLEAUS_X + CARD_X_STRIDE * 6,
                TABLEAUS_Y,
                tableau7_cards,
                CardSinks::Tableau7,
                CardSources::Tableau7,
            ),
            held_card: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MouseUp(mouse_x, mouse_y) => {
                let mut result = false;
                if let Some(source) = self.held_card.as_ref().map(HeldCard::source) {
                    let card = self
                        .borrow_source(source)
                        .peek_card()
                        .expect("card should be available");
                    if let Some(sink) = self
                        .borrow_sinks()
                        .iter_mut()
                        .find(|s| {
                            s.within_bounds(mouse_x, mouse_y) && s.is_placement_possible(vec![card])
                        })
                        .map(|s| s.card_sink())
                    {
                        // Place card
                        let source = self.borrow_source(source);
                        let physical_card = source.take_card().expect("card should be available");
                        self.borrow_sink(sink)
                            .place_card(mouse_x, mouse_y, vec![physical_card])
                            .expect("placement should be possible");

                        self.held_card = None;
                        result = true;
                    } else {
                        // Return card
                        let physical_card = self
                            .borrow_source(source)
                            .borrow_card_mut()
                            .expect("card should be available");

                        physical_card.set_visible(true);
                        physical_card.set_prev_loc(
                            mouse_x - CARD_WIDTH as i32 / 2,
                            mouse_y - CARD_HEIGHT as i32 / 2,
                        );
                        self.held_card = None;
                        result = true;
                    }
                }

                result
            }
            Msg::MouseDown(x, y) => {
                self.stock_discard.handle_click(x, y) || {
                    let mut result = false;
                    if self.held_card.is_none() {
                        if let Some(source) = self.borrow_sources().iter_mut().find(|s| {
                            s.borrow_card()
                                .map(|c| c.within_bounds(x, y))
                                .unwrap_or(false)
                        }) {
                            source
                                .borrow_card_mut()
                                .expect("card should be available")
                                .set_visible(false);
                            result = true;
                            self.held_card = Some(HeldCard::new(*source, x, y));
                        } else {
                            self.held_card = None;
                        }
                    }
                    result
                }
            }
            Msg::MouseMove(x, y, _dx, _dy) => {
                if let Some(held_card) = &mut self.held_card {
                    held_card.set_mouse_position(x, y);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let held_card_html = self.held_card.as_ref().map_or(html! {}, |c| c.as_html());
        html! {
            <div>
                /* <button onclick=self.link.callback(|_| Msg::StockToDiscard)>{ "Deal" }</button> */
                <br/>
                { self.foundation1.as_html() }
                { self.foundation2.as_html() }
                { self.foundation3.as_html() }
                { self.foundation4.as_html() }
                { self.tableau1.as_html() }
                { self.tableau2.as_html() }
                { self.tableau3.as_html() }
                { self.tableau4.as_html() }
                { self.tableau5.as_html() }
                { self.tableau6.as_html() }
                { self.tableau7.as_html() }
                { self.stock_discard.as_html() }
                { held_card_html }
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

fn main() {
    yew::start_app::<Model>();
}
