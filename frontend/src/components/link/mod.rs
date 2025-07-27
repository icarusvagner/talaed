use leptos::prelude::*;
use tailwind_fuse::*;

#[component]
pub fn Link(
	#[prop(into, optional)] variant: RwSignal<LinkVariant>,
	#[prop(into, optional)] size: Signal<LinkSize>,
	#[prop(into, optional)] class: Signal<String>,
	#[prop(into, optional)] route: MaybeProp<String>,
	children: Children,
) -> AnyView {
	let class = Memo::new(move |_| {
		let variant = variant.get();
		let size = size.get();
		let link = LinkClass { variant, size };
		link.with_class(class.get())
	});

	view! {
		<a href=route class=class>
			{children()}
		</a>
	}
	.into_any()
}

#[derive(TwClass, Clone, Copy)]
pub struct LinkClass {
	pub variant: LinkVariant,
	pub size: LinkSize,
}

#[derive(TwVariant)]
pub enum LinkVariant {
	#[tw(
		default,
		class = "bg-transparent duration-200 ease-in-out w-full text-center hover:bg-stone-500/10 text-stone-800 inline-flex items-center gap-2 font-montserrat font-bold"
	)]
	Default,
	#[tw(
		class = "bg-primary text-stone-50 font-medium tracking-wide duration-200 ease-in-out w-full text-center hover:bg-[var(--red-400)] inline-flex items-center gap-2 font-montserrat font-bold"
	)]
	Primary,
	#[tw(
		class = "bg-secondary text-stone-50 font-medium tracking-wide duration-200 ease-in-out w-full text-center hover:bg-[var(--purple-500)] inline-flex items-center gap-2 font-montserrat font-bold"
	)]
	Secondary,
}

#[derive(TwVariant)]
pub enum LinkSize {
	#[tw(default, class = "h-12 px-5 py-2")]
	Default,
	#[tw(class = "h-8 rounded-md py-3 text-xs")]
	Sm,
	#[tw(class = "h-14 rounded-md py-2 px-5 text-xl")]
	Lg,
	#[tw(class = "h-9 w-9")]
	Icon,
}
