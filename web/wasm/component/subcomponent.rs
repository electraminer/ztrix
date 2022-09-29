use yew::prelude::*;

pub trait Subcomponent {
	type Component: yew::Component;
	type Properties<'a>;

	fn new() -> Self;

	fn view<'a>(&self, ctx: &Context<Self::Component>,
		props: Self::Properties<'a>) -> Html;

	fn rendered<'a>(&self, ctx: &Context<Self::Component>,
		props: Self::Properties<'a>, first: bool);
}