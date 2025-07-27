use leptos::{either::Either, prelude::*};
use phosphor_leptos::{Icon, USER};

#[component]
pub fn Avatar(
	#[prop(into, optional)] src: MaybeProp<String>,
	#[prop(into, optional)] size: MaybeProp<u8>,
) -> AnyView {
	let size = move || size.read().unwrap_or(10);
	view! {
		<div class=format!(
			"overflow-hidden rounded-full size-{}",
			size(),
		)>
			{move || {
				if src.read().is_some() {
					Either::Left(
						view! {
							<img src=src class="object-contain w-full h-full" />
						},
					)
				} else {
					Either::Right(
						view! {
							<div class="inline-flex justify-center items-center p-1.5 bg-stone-200">
								<Icon icon=USER attr:class="h-6 w-6" />
							</div>
						},
					)
				}
			}}
		</div>
	}
	.into_any()
}
