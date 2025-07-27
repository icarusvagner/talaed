use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn MainLayout() -> AnyView {
	view! {
		<section class="min-h-screen">
			<Outlet />
		</section>
	}
	.into_any()
}
