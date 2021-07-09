use foundation::Foundation;
use rand::{prelude::SliceRandom, thread_rng};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::MouseEvent;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod card;
mod foundation;
mod stock_discard;
mod util;

use card::{Card, CardSink, CardSource, CardVisual, PhysicalCard, DECK};
use stock_discard::StockDiscard;

const CARD_WIDTH: u32 = 125;
const CARD_HEIGHT: u32 = 175;

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

#[derive(Clone, Copy)]
struct HeldCard {
    card: Card,
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
            card,
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
            CardVisual::Card(self.card).as_draggable_html_from(from_x, from_y, self.x, self.y)
        } else {
            CardVisual::Card(self.card).as_draggable_html(self.x, self.y)
        }
    }
}

struct Model {
    stock_discard: StockDiscard,
    foundation1: Foundation,
    foundation2: Foundation,
    foundation3: Foundation,
    foundation4: Foundation,
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
        ]
    }

    fn borrow_sinks(&mut self) -> Vec<&mut dyn CardSink> {
        vec![
            &mut self.foundation1,
            &mut self.foundation2,
            &mut self.foundation3,
            &mut self.foundation4,
        ]
    }

    fn borrow_source(&mut self, source: CardSources) -> &mut dyn CardSource {
        match source {
            CardSources::Discard => self.stock_discard.discard_mut(),
            CardSources::Foundation1 => &mut self.foundation1,
            CardSources::Foundation2 => &mut self.foundation2,
            CardSources::Foundation3 => &mut self.foundation3,
            CardSources::Foundation4 => &mut self.foundation4,
            CardSources::Tableau1 => todo!(),
            CardSources::Tableau2 => todo!(),
            CardSources::Tableau3 => todo!(),
            CardSources::Tableau4 => todo!(),
            CardSources::Tableau5 => todo!(),
            CardSources::Tableau6 => todo!(),
            CardSources::Tableau7 => todo!(),
        }
    }

    fn borrow_sink(&mut self, sink: CardSinks) -> &mut dyn CardSink {
        match sink {
            CardSinks::Foundation1 => &mut self.foundation1,
            CardSinks::Foundation2 => &mut self.foundation2,
            CardSinks::Foundation3 => &mut self.foundation3,
            CardSinks::Foundation4 => &mut self.foundation4,
            CardSinks::Tableau1 => todo!(),
            CardSinks::Tableau2 => todo!(),
            CardSinks::Tableau3 => todo!(),
            CardSinks::Tableau4 => todo!(),
            CardSinks::Tableau5 => todo!(),
            CardSinks::Tableau6 => todo!(),
            CardSinks::Tableau7 => todo!(),
        }
    }

    fn borrow_held_source(&mut self, held_card: HeldCard) -> &mut dyn CardSource {
        self.borrow_source(held_card.source())
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut stock_cards: Vec<PhysicalCard> =
            DECK.iter().map(|c| PhysicalCard::new(0, 0, c)).collect();
        stock_cards.shuffle(&mut thread_rng());

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

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

        Self {
            stock_discard: StockDiscard::from_cards(10, 10, stock_cards),
            foundation1: Foundation::new(400, 10, CardSinks::Foundation1, CardSources::Foundation1),
            foundation2: Foundation::new(550, 10, CardSinks::Foundation2, CardSources::Foundation2),
            foundation3: Foundation::new(700, 10, CardSinks::Foundation3, CardSources::Foundation3),
            foundation4: Foundation::new(850, 10, CardSinks::Foundation4, CardSources::Foundation4),
            held_card: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MouseUp(mouse_x, mouse_y) => {
                let mut result = false;
                if let Some(held_card) = self.held_card {
                    let card = self
                        .borrow_held_source(held_card)
                        .peek_card()
                        .expect("card should be available");
                    if let Some(sink) = self
                        .borrow_sinks()
                        .iter_mut()
                        .find(|s| {
                            s.within_bounds(mouse_x, mouse_y) && s.is_placement_possible(card)
                        })
                        .map(|s| s.card_sink())
                    {
                        // Place card
                        let source = self.borrow_source(held_card.source());
                        let physical_card = source.take_card().expect("card should be available");
                        self.borrow_sink(sink)
                            .place_card(mouse_x, mouse_y, physical_card)
                            .expect("placement should be possible");

                        self.held_card = None;
                        result = true;
                    } else {
                        // Return card
                        let physical_card = self
                            .borrow_held_source(held_card)
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
                    self.held_card = self.held_card.or_else(|| {
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
                            Some(HeldCard::new(*source, x, y))
                        } else {
                            None
                        }
                    });
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
        let held_card_html = self.held_card.map_or(html! {}, |c| c.as_html());
        html! {
            <div>
                /* <button onclick=self.link.callback(|_| Msg::StockToDiscard)>{ "Deal" }</button> */
                <br/>
                { self.foundation1.as_html() }
                { self.foundation2.as_html() }
                { self.foundation3.as_html() }
                { self.foundation4.as_html() }
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
