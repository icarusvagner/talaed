use leptos::prelude::*;
use leptos_router::components::A;
use phosphor_leptos::{Icon, PLUS};

use crate::components::avatar::Avatar;

#[component]
pub fn Navbar() -> AnyView {
	view! {
		<nav class="flex gap-5 items-center py-6 px-5 w-full bg-stone-100">
			<A
				href="/home"
				attr:class="inline-flex itemsc-enter font-black font-montserrat text-5xl gap-0.5"
			>
				"Tala"
				<img src="/logos/ED-logo.png" class="object-contain h-10" />
			</A>
			<div class="mx-auto"></div>
			<button class="cursor-pointer">
				<Icon icon=PLUS attr:class="h-8 w-8 font-bold" />
			</button>
			<Avatar size=12 />
		</nav>
	}
	.into_any()
}
