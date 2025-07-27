use leptoaster::{Toaster, provide_toaster};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{StaticSegment, components::*};

/// Modules
mod components;
mod layouts;
mod pages;

/// Top-Level pages
use crate::{layouts::*, pages::*};

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();
	provide_toaster();

	view! {
		<Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

		// sets the document title
		<Title text="Welcome to TalaED" />

		// injects metadata in the <head> of the page
		<Meta charset="UTF-8" />
		<Meta name="viewport" content="width=device-width, initial-scale=1.0" />

		// inject toaster to the root body
		<Toaster stacked=true />

		<Router>
			<Routes fallback=|| view! { <NotFound /> }>
				<Route path=StaticSegment("login") view=LoginRoute />
				<Route path=StaticSegment("getstarted") view=RegisterRoute />
				<ParentRoute path=StaticSegment("home") view=MainLayout>
					<Route path=StaticSegment("") view=Home />
				</ParentRoute>
			</Routes>
		</Router>
	}
}
