use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::{Form, A};

#[component]
pub fn RegisterRoute() -> impl IntoView {
	view! {
		<Title text="Get Started now" />

		<section class="grid py-10 px-6 min-h-screen lg:grid-cols-2 lg:py-0 lg:px-0">
			<div class="flex flex-col gap-10 justify-center mx-auto w-full md:w-3xl lg:w-xl">
				<div class="flex gap-1 justify-center items-center">
					<h1 class="text-5xl font-black tracking-widest text-center font-montserrat">
						"Tala"
					</h1>
					<img
						src="/logos/ED-logo.png"
						alt="TalaED"
						class="object-contain w-14 lg:w-18"
					/>
				</div>

				<h1 class="text-xl font-medium text-center sm:text-2xl md:text-3xl lg:text-4xl text-stone-800">
					"Unlock Knowledge. Discover Possibilities. Learn with TalaED."
				</h1>
				<Form action="#" attr:class="flex flex-col gap-6 w-full">
					<input
						type="email"
						placeholder="Your email"
						autocomplete="email"
						class="p-3 w-full text-lg rounded border duration-200 ease-in-out outline-none focus:border-green-500 focus:shadow-2xl peer border-neutral-500 text-neutral-800"
					/>
					<input
						type="password"
						placeholder="Your password"
						autocomplete="new-password"
						class="p-3 w-full text-lg rounded border duration-200 ease-in-out outline-none focus:border-green-500 focus:shadow-2xl peer border-neutral-500 text-neutral-800"
					/>

					<input
						type="password"
						placeholder="Confirm your password"
						autocomplete="current-password"
						class="p-3 w-full text-lg rounded border duration-200 ease-in-out outline-none focus:border-green-500 focus:shadow-2xl peer border-neutral-500 text-neutral-800"
					/>

					<button
						type="submit"
						class="p-3 w-full text-lg font-bold tracking-wider text-white bg-teal-600 rounded duration-200 ease-in-out cursor-pointer hover:bg-teal-500"
					>
						"Start your learning now!"
					</button>
				</Form>

				<span class="tracking-wide text-stone-600">
					"By signing up, you agree to the "
					<A href="#" attr:class="font-medium text-teal-600">
						"Terms and Conditions"
					</A>" and the "
					<A href="#" attr:class="font-medium text-teal-600">
						"Privacy Policy"
					</A>
				</span>

				<span class="text-base text-stone-600">
					"Already have account? "
					<A
						href="/login"
						attr:class="text-teal-600 duration-200 ease-in-out hover:underline"
					>
						"Login now"
					</A>
				</span>
			</div>
			<div class="hidden relative w-full h-full lg:flex">
				<img
					src="/svgs/Education-amico.svg"
					alt="education teaching"
					class="object-contain w-2xl"
				/>
			</div>
		</section>
	}
}
