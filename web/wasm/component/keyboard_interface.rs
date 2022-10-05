
use web_sys::HtmlElement;
use std::collections::HashSet;





use controller::input_handler::ButtonEvent;



use yew::prelude::*;



use gloo_timers::callback::Interval;

pub enum Msg {
	Button(ButtonEvent<String>),
	LostFocus,
	GainedFocus,
	Interval,
}

#[derive(Properties, PartialEq)]
pub struct Props {
	#[prop_or_default]
	pub children: Children,
	#[prop_or_default]
	pub onkey: Callback<ButtonEvent<String>>
}

pub struct KeyboardInterface {
	pressed: HashSet<String>,
	focused: bool,
	_interval: Interval,
	node_ref: NodeRef,
}

impl Component for KeyboardInterface {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
		let link = ctx.link().clone();
        Self {
        	pressed: HashSet::new(),
        	focused: true,
        	_interval: Interval::new(16, move ||
				link.send_message(Msg::Interval)),
        	node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>,
    		msg: Msg) -> bool {
    	match msg {
    		Msg::Button(event) => match event {
    			ButtonEvent::Press(button) =>
    				if self.pressed.insert(button.clone()) {
						ctx.props().onkey.emit(
							ButtonEvent::Press(button));
    				}
    			ButtonEvent::Release(button) =>
    				if self.pressed.remove(&button) {
						ctx.props().onkey.emit(
							ButtonEvent::Release(button));
    				}
    		}
			Msg::LostFocus => self.focused = false,
			Msg::GainedFocus => self.focused = true,
			Msg::Interval => if !self.focused {
				for button in self.pressed.drain() {
					ctx.props().onkey.emit(
						ButtonEvent::Release(button));
				}
			}
    	}
    	return false;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
    	html! {
        	<div class="interface" tabindex=1
        		ref={self.node_ref.clone()}
            	onkeydown={ctx.link().callback(
            		move |e: KeyboardEvent| Msg::Button(
            			ButtonEvent::Press(e.code())))}
            	onkeyup={ctx.link().callback(
            		move |e: KeyboardEvent| Msg::Button(
            			ButtonEvent::Release(e.code())))}
            	onfocusout={ctx.link().callback(
            		move |_| Msg::LostFocus)}
            	onfocusin={ctx.link().callback(
            		move |_| Msg::GainedFocus)}>
            	{for ctx.props().children.iter()}
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first: bool) {
    	if first {
		    let elem = self.node_ref.cast::<HtmlElement>()
		    	.expect("element should be an html element");
		    elem.focus().expect("should be able to focus");
    	}
    }
}
