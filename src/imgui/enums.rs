use super::raw::types::*;

pub enum ImGuiWindowFlags {
	None = 0,
	NoTitleBar = ImGuiWindowFlags_::NoTitleBar as isize,
	NoResize = ImGuiWindowFlags_::NoResize as isize,
	NoMove = ImGuiWindowFlags_::NoMove as isize,
	NoScrollbar = ImGuiWindowFlags_::NoScrollbar as isize,
	NoScrollWithMouse = ImGuiWindowFlags_::NoScrollWithMouse as isize,
	NoCollapse = ImGuiWindowFlags_::NoCollapse as isize,
	AlwaysAutoResize = ImGuiWindowFlags_::AlwaysAutoResize as isize,
	ShowBorders = ImGuiWindowFlags_::ShowBorders as isize,
	NoSavedSettings = ImGuiWindowFlags_::NoSavedSettings as isize,
	NoInputs = ImGuiWindowFlags_::NoInputs as isize,
	MenuBar = ImGuiWindowFlags_::MenuBar as isize,
	HorizontalScrollbar = ImGuiWindowFlags_::HorizontalScrollbar as isize,
	NoFocusOnAppearing = ImGuiWindowFlags_::NoFocusOnAppearing as isize,
	NoBringToFrontOnFocus = ImGuiWindowFlags_::NoBringToFrontOnFocus as isize,
	AlwaysVerticalScrollbar = ImGuiWindowFlags_::AlwaysVerticalScrollbar as isize,
	AlwaysHorizontalScrollbar = ImGuiWindowFlags_::AlwaysHorizontalScrollbar as isize,
	AlwaysUseWindowPadding = ImGuiWindowFlags_::AlwaysUseWindowPadding as isize,
	ChildWindow = ImGuiWindowFlags_::ChildWindow as isize,
	ChildWindowAutoFitX = ImGuiWindowFlags_::ChildWindowAutoFitX as isize,
	ChildWindowAutoFitY = ImGuiWindowFlags_::ChildWindowAutoFitY as isize,
	ComboBox = ImGuiWindowFlags_::ComboBox as isize,
	Tooltip = ImGuiWindowFlags_::Tooltip as isize,
	Popup = ImGuiWindowFlags_::Popup as isize,
	Modal = ImGuiWindowFlags_::Modal as isize,
	ChildMenu = ImGuiWindowFlags_::ChildMenu as isize,
}

impl ImGuiWindowFlags {
	pub fn as_c(self) -> ImGuiWindowFlags_::Type { self as ImGuiWindowFlags_::Type }
}

pub enum ImGuiSetCond {
	Always = ImGuiSetCond_::Always as isize,
	Once = ImGuiSetCond_::Once as isize,
	FirstUseEver = ImGuiSetCond_::FirstUseEver as isize,
	Appearing = ImGuiSetCond_::Appearing as isize,
}

impl ImGuiSetCond {
	pub fn as_c(self) -> ImGuiSetCond_::Type { self as ImGuiSetCond_::Type }
}

pub enum ImGuiCol {
	Text = ImGuiCol_::Text as isize,
	TextDisabled = ImGuiCol_::TextDisabled as isize,
	WindowBg = ImGuiCol_::WindowBg as isize,
	ChildWindowBg = ImGuiCol_::ChildWindowBg as isize,
	PopupBg = ImGuiCol_::PopupBg as isize,
	Border = ImGuiCol_::Border as isize,
	BorderShadow = ImGuiCol_::BorderShadow as isize,
	FrameBg = ImGuiCol_::FrameBg as isize,
	FrameBgHovered = ImGuiCol_::FrameBgHovered as isize,
	FrameBgActive = ImGuiCol_::FrameBgActive as isize,
	TitleBg = ImGuiCol_::TitleBg as isize,
	TitleBgCollapsed = ImGuiCol_::TitleBgCollapsed as isize,
	TitleBgActive = ImGuiCol_::TitleBgActive as isize,
	MenuBarBg = ImGuiCol_::MenuBarBg as isize,
	ScrollbarBg = ImGuiCol_::ScrollbarBg as isize,
	ScrollbarGrab = ImGuiCol_::ScrollbarGrab as isize,
	ScrollbarGrabHovered = ImGuiCol_::ScrollbarGrabHovered as isize,
	ScrollbarGrabActive = ImGuiCol_::ScrollbarGrabActive as isize,
	ComboBg = ImGuiCol_::ComboBg as isize,
	CheckMark = ImGuiCol_::CheckMark as isize,
	SliderGrab = ImGuiCol_::SliderGrab as isize,
	SliderGrabActive = ImGuiCol_::SliderGrabActive as isize,
	Button = ImGuiCol_::Button as isize,
	ButtonHovered = ImGuiCol_::ButtonHovered as isize,
	ButtonActive = ImGuiCol_::ButtonActive as isize,
	Header = ImGuiCol_::Header as isize,
	HeaderHovered = ImGuiCol_::HeaderHovered as isize,
	HeaderActive = ImGuiCol_::HeaderActive as isize,
	Column = ImGuiCol_::Column as isize,
	ColumnHovered = ImGuiCol_::ColumnHovered as isize,
	ColumnActive = ImGuiCol_::ColumnActive as isize,
	ResizeGrip = ImGuiCol_::ResizeGrip as isize,
	ResizeGripHovered = ImGuiCol_::ResizeGripHovered as isize,
	ResizeGripActive = ImGuiCol_::ResizeGripActive as isize,
	CloseButton = ImGuiCol_::CloseButton as isize,
	CloseButtonHovered = ImGuiCol_::CloseButtonHovered as isize,
	CloseButtonActive = ImGuiCol_::CloseButtonActive as isize,
	PlotLines = ImGuiCol_::PlotLines as isize,
	PlotLinesHovered = ImGuiCol_::PlotLinesHovered as isize,
	PlotHistogram = ImGuiCol_::PlotHistogram as isize,
	PlotHistogramHovered = ImGuiCol_::PlotHistogramHovered as isize,
	TextSelectedBg = ImGuiCol_::TextSelectedBg as isize,
	ModalWindowDarkening = ImGuiCol_::ModalWindowDarkening as isize,
}

impl ImGuiCol {
	pub fn as_c(self) -> ImGuiCol_::Type { self as i32 }
}

pub enum ImGuiStyleVar {
	Alpha = ImGuiStyleVar_::Alpha as isize,
	WindowPadding = ImGuiStyleVar_::WindowPadding as isize,
	WindowRounding = ImGuiStyleVar_::WindowRounding as isize,
	WindowMinSize = ImGuiStyleVar_::WindowMinSize as isize,
	ChildWindowRounding = ImGuiStyleVar_::ChildWindowRounding as isize,
	FramePadding = ImGuiStyleVar_::FramePadding as isize,
	FrameRounding = ImGuiStyleVar_::FrameRounding as isize,
	ItemSpacing = ImGuiStyleVar_::ItemSpacing as isize,
	ItemInnerSpacing = ImGuiStyleVar_::ItemInnerSpacing as isize,
	IndentSpacing = ImGuiStyleVar_::IndentSpacing as isize,
	GrabMinSize = ImGuiStyleVar_::GrabMinSize as isize,
}

impl ImGuiStyleVar {
	pub fn as_c(self) -> ImGuiStyleVar_::Type { self as ImGuiStyleVar_::Type }
}

pub enum ImGuiColorEditMode {
	UserSelect = ImGuiColorEditMode_::UserSelect as isize,
	UserSelectShowButton = ImGuiColorEditMode_::UserSelectShowButton as isize,
	RGB = ImGuiColorEditMode_::RGB as isize,
	HSV = ImGuiColorEditMode_::HSV as isize,
	HEX = ImGuiColorEditMode_::HEX as isize,
}

impl ImGuiColorEditMode {
	pub fn as_c(self) -> ImGuiColorEditMode_::Type { self as ImGuiColorEditMode_::Type }
}

pub enum ImGuiInputTextFlags {
	CharsDecimal = ImGuiInputTextFlags_::CharsDecimal as isize,
	CharsHexadecimal = ImGuiInputTextFlags_::CharsHexadecimal as isize,
	CharsUppercase = ImGuiInputTextFlags_::CharsUppercase as isize,
	CharsNoBlank = ImGuiInputTextFlags_::CharsNoBlank as isize,
	AutoSelectAll = ImGuiInputTextFlags_::AutoSelectAll as isize,
	EnterReturnsTrue = ImGuiInputTextFlags_::EnterReturnsTrue as isize,
	CallbackCompletion = ImGuiInputTextFlags_::CallbackCompletion as isize,
	CallbackHistory = ImGuiInputTextFlags_::CallbackHistory as isize,
	CallbackAlways = ImGuiInputTextFlags_::CallbackAlways as isize,
	CallbackCharFilter = ImGuiInputTextFlags_::CallbackCharFilter as isize,
	AllowTabInput = ImGuiInputTextFlags_::AllowTabInput as isize,
	CtrlEnterForNewLine = ImGuiInputTextFlags_::CtrlEnterForNewLine as isize,
	NoHorizontalScroll = ImGuiInputTextFlags_::NoHorizontalScroll as isize,
	AlwaysInsertMode = ImGuiInputTextFlags_::AlwaysInsertMode as isize,
	ReadOnly = ImGuiInputTextFlags_::ReadOnly as isize,
	Password = ImGuiInputTextFlags_::Password as isize,
	Multiline = ImGuiInputTextFlags_::Multiline as isize,
}

impl ImGuiInputTextFlags {
	pub fn as_c(self) -> ImGuiInputTextFlags_::Type { self as ImGuiInputTextFlags_::Type }
}

pub enum ImGuiSelectableFlags {
	DontClosePopups = ImGuiSelectableFlags_::DontClosePopups as isize,
	SpanAllColumns = ImGuiSelectableFlags_::SpanAllColumns as isize,
	AllowDoubleClick = ImGuiSelectableFlags_::AllowDoubleClick as isize,
}

impl ImGuiSelectableFlags {
	pub fn as_c(self) -> ImGuiSelectableFlags_::Type { self as ImGuiSelectableFlags_::Type }
}

pub enum ImGuiKey {
	Tab = ImGuiKey_::Tab as isize,
	LeftArrow = ImGuiKey_::LeftArrow as isize,
	RightArrow = ImGuiKey_::RightArrow as isize,
	UpArrow = ImGuiKey_::UpArrow as isize,
	DownArrow = ImGuiKey_::DownArrow as isize,
	PageUp = ImGuiKey_::PageUp as isize,
	PageDown = ImGuiKey_::PageDown as isize,
	Home = ImGuiKey_::Home as isize,
	End = ImGuiKey_::End as isize,
	Delete = ImGuiKey_::Delete as isize,
	Backspace = ImGuiKey_::Backspace as isize,
	Enter = ImGuiKey_::Enter as isize,
	Escape = ImGuiKey_::Escape as isize,
	A = ImGuiKey_::A as isize,
	C = ImGuiKey_::C as isize,
	V = ImGuiKey_::V as isize,
	X = ImGuiKey_::X as isize,
	Y = ImGuiKey_::Y as isize,
	Z = ImGuiKey_::Z as isize,
}

impl ImGuiKey {
	pub fn as_c(self) -> ImGuiKey_::Type { self as ImGuiKey_::Type }
}

pub enum ImGuiMouseCursor {
	Arrow = ImGuiMouseCursor_::Arrow as isize,
	TextInput = ImGuiMouseCursor_::TextInput as isize,
	Move = ImGuiMouseCursor_::Move as isize,
	ResizeNS = ImGuiMouseCursor_::ResizeNS as isize,
	ResizeEW = ImGuiMouseCursor_::ResizeEW as isize,
	ResizeNESW = ImGuiMouseCursor_::ResizeNESW as isize,
	ResizeNWSE = ImGuiMouseCursor_::ResizeNWSE as isize,
}

impl ImGuiMouseCursor {
	pub fn as_c(self) -> ImGuiMouseCursor_::Type { self as ImGuiMouseCursor_::Type }

	pub fn from_c(a: ImGuiMouseCursor_::Type) -> ImGuiMouseCursor {
		match a {
			ImGuiMouseCursor_::Arrow => ImGuiMouseCursor::Arrow,
			ImGuiMouseCursor_::TextInput => ImGuiMouseCursor::TextInput,
			ImGuiMouseCursor_::Move => ImGuiMouseCursor::Move,
			ImGuiMouseCursor_::ResizeNS => ImGuiMouseCursor::ResizeNS,
			ImGuiMouseCursor_::ResizeEW => ImGuiMouseCursor::ResizeEW,
			ImGuiMouseCursor_::ResizeNESW => ImGuiMouseCursor::ResizeNESW,
			ImGuiMouseCursor_::ResizeNWSE => ImGuiMouseCursor::ResizeNWSE,
			_ => unreachable!()
		}
	}
}