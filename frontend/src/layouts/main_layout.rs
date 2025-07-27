use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::{header::Navbar, siderbar::Siderbar};

#[component]
pub fn MainLayout() -> AnyView {
	view! {
		<main class="relative bg-stone-100">
			<Navbar />
			<div class="flex">
				<Siderbar />
				<div class="flex flex-col flex-1 px-6 bg-stone-50 rounded-[40px_5px_5px_5px]">
					<div class="w-full">
						<Outlet />
					</div>
				</div>
			</div>
		</main>
	}
	.into_any()
}
