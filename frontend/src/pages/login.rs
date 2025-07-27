use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::{Form, A};

#[component]
pub fn LoginRoute() -> impl IntoView {
	view! {
		<Title text="Sign-in to EDumentor" />

		<section class="flex flex-col justify-center items-center min-h-screen">
			<div class="flex flex-col gap-10 justify-center items-center mx-auto w-4xl">
				<div class="flex gap-1 justify-center items-center">
					<h1 class="text-5xl font-black tracking-widest text-center font-montserrat">
						"Tala"
					</h1>
					<img
						src="/logos/ED-logo.png"
						alt="TalaED"
						class="object-contain w-18"
					/>
				</div>

				<div class="flex w-full">
					<div class="flex flex-col gap-6 p-5 min-w-[447px]">
						<h1 class="text-2xl font-bold">"Login your account"</h1>

						<Form action="#" attr:class="flex flex-col gap-6">
							<input
								type="email"
								placeholder="Your e-mail"
								autocomplete="email"
								class="p-3 w-full text-lg rounded border duration-200 ease-in-out outline-none focus:border-green-500 focus:shadow-2xl peer border-neutral-500 text-neutral-800"
							/>
							<input
								type="password"
								placeholder="Your password"
								autocomplete="new-password"
								class="p-3 w-full text-lg rounded border duration-200 ease-in-out outline-none focus:border-green-500 focus:shadow-2xl peer border-neutral-500 text-neutral-800"
							/>

							<div class="flex gap-1 text-stone-500">
								<input
									type="checkbox"
									name="remember_me"
									id="remember_me"
								/>
								<label for="remember_me" class="text-base">
									"Remember me"
								</label>
							</div>

							<button
								type="submit"
								class="p-3 text-lg font-bold tracking-wider text-white bg-teal-600 rounded duration-200 ease-in-out cursor-pointer hover:bg-teal-500"
							>
								"Login"
							</button>
						</Form>

						<A
							href="#"
							attr:class="text-sm text-teal-600 duration-200 ease-in-out hover:underline"
						>
							"Forgot Your Password?"
						</A>
					</div>

					<div class="w-0.5 bg-stone-500/20"></div>

					<div class="flex flex-col gap-6 p-5">
						<h1 class="text-2xl font-bold">"...or register now!"</h1>
						<span class="text-base text-stone-600">
							"Every journey begins with a single step, thank you for taking yours with us. By registrering today, you're unlocking new opportunities to grow, connect and succeed. Let's make learning meaningful together."
						</span>
						<A
							href="/getstarted"
							attr:class="p-3 text-lg font-bold text-center tracking-wider text-white bg-teal-600 rounded duration-200 ease-in-out cursor-pointer hover:bg-teal-500"
						>
							"Sign up today"
						</A>

					</div>
				</div>
			</div>
		</section>
	}
}
