
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub name: String,
	pub value: String,

	#[prop_or_default]
	pub onchange: Callback<String>,
}

#[function_component(FieldConfig)]
pub fn field_config(props: &Props) -> Html {
	let node_ref = use_node_ref();
	html! {
		<div class="thin-row">
			<h3>{props.name.clone()}</h3>
			<input ref={node_ref.clone()}
				value={props.value.clone()}
				onchange={props.onchange.reform(move |_| {
					node_ref.cast::<HtmlInputElement>()
		    			.expect("element should be an input")
	    				.value()
				})}/>
		</div>
	}
}