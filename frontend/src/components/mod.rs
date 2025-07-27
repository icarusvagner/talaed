pub mod avatar;
pub mod link;
pub mod counter_btn;
pub mod header;
pub mod siderbar;

const SIDEBAR_LINKS: [SidebarLinksData; 4] = [
	SidebarLinksData {
		label: "Home",
		icon: icondata::RiHome5BuildingsLine,
		route: "/home",
	},
	SidebarLinksData {
		label: "Calendar",
		icon: icondata::MdiCalendarMultiselectOutline,
		route: "/home/calendar",
	},
	SidebarLinksData {
		label: "Class",
		icon: icondata::MdiGoogleClassroom,
		route: "/home/class",
	},
	SidebarLinksData {
		label: "Exams",
		icon: icondata::MdiClipboardTextMultipleOutline,
		route: "/home/exam",
	},
];

#[derive(Clone, Debug)]
pub struct SidebarLinksData {
	label: &'static str,
	icon: icondata_core::Icon,
	route: &'static str,
}
