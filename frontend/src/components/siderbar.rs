use leptos::prelude::*;
use leptos_icons::Icon;

use crate::components::{SIDEBAR_LINKS, link::Link};

#[component]
pub fn Siderbar() -> AnyView {
	let sidebar_links = RwSignal::new(SIDEBAR_LINKS);

	view! {
		<aside class="flex sticky top-0 left-0 flex-col justify-between p-6 h-screen max-sm:hidden w-fit bg-stone-100 font-montserrat lg:w-[264px]">
			<div class="flex flex-col flex-1 gap-6">
				<For
					each=move || sidebar_links.get()
					key=|links| links.label
					let:child
				>
					<Link route=child.route>
						<Icon icon=child.icon attr:class="h-8 w-8" />
						<p class="text-lg font-semibold max-lg:hidden">
							{child.label}
						</p>
					</Link>
				</For>
			</div>
		</aside>
	}
    .into_any()
}
