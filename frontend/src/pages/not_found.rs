use leptos::prelude::*;
use leptos_router::components::A;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
	view! {
		<section class="grid gap-5 p-5 mx-auto max-w-5xl min-h-screen md:grid-cols-2">
			<div class="flex flex-col order-2 gap-5 justify-center md:order-1">
				<h1 class="text-7xl font-black tracking-widest text-stone-800">
					"Uh oh!..."
				</h1>
				<span class="text-2xl text-stone-500">
					"Seems like the page you are looking for not found or having problem with the page?"
				</span>
				<div class="grid gap-5 w-full md:grid-cols-2">
					<A
						href="#"
						attr:class="inline-flex items-center justify-center bg-teal-500 rounded px-5 py-2.5 text-xl font-semibold text-stone-50 duration-200 ease-in-out hover:bg-teal-600 tracking-wide"
					>
						"Contact Us"
					</A>
					<A
						href="/home"
						attr:class="inline-flex items-center justify-center border border-teal-500 rounded px-5 py-2.5 text-xl font-semibold text-teal-500 duration-200 ease-in-out hover:text-stone-50 hover:bg-teal-600 tracking-wide"
					>
						"Go Home"
					</A>
				</div>
			</div>
			<div class="order-1 w-full h-full md:order-2">
				<img
					src="/svgs/404-error-with-a-cute-animal-animate.svg"
					alt="Cute cat"
					class="object-contain w-full h-full"
				/>
			</div>
		</section>
	}
}
