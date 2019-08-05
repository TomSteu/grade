
pub mod macros {

	// A helper macro to find Builders for Widgets etc.
	#[doc(hidden)]
	#[macro_export]
	macro_rules! __grade_find_builder_internal {
	    (AboutDialog) => {gtk::AboutDialogBuilder};
	    (AccelLabel) => {gtk::AccelLabelBuilder};
	    (ActionBar) => {gtk::ActionBarBuilder};
	    (AppChooserButton) => {gtk::AppChooserButtonBuilder};
	    (AppChooserDialog) => {gtk::AppChooserDialogBuilder};
	    (AppChooserWidget) => {gtk::AppChooserWidgetBuilder};
	    (Application) => {gtk::ApplicationBuilder};
	    (ApplicationWindow) => {gtk::ApplicationWindowBuilder};
	    (AspectFrame) => {gtk::AspectFrameBuilder};
	    (Assistant) => {gtk::AssistantBuilder};
	    (Box) => {gtk::BoxBuilder};
	    (Button) => {gtk::ButtonBuilder};
	    (ButtonBox) => {gtk::ButtonBoxBuilder};
	    (Calendear) => {gtk::CalendearBuilder};
	    (CellAreaBox) => {gtk::CellAreaBoxBuilder};
	    (CellRendererAccel) => {gtk::CellRendererAccelBuilder};
	    (CellRendererCombo) => {gtk::CellRendererComboBuilder};
	    (CellRendererPixbuf) => {gtk::CellRendererPixbufBuilder};
	    (CellRendererProgress) => {gtk::CellRendererProgressBuilder};
	    (CellRendererSpin) => {gtk::CellRendererSpinBuilder};
	    (CellRendererSpinner) => {gtk::CellRendererSpinnerBuilder};
	    (CellRendererText) => {gtk::CellRendererTextBuilder};
	    (CellRendererToggle) => {gtk::CellRendererToggleBuilder};
	    (CellView) => {gtk::CellViewBuilder};
	    (CheckButton) => {gtk::CheckButtonBuilder};
	    (CheckMenuItem) => {gtk::CheckMenuItemBuilder};
	    (ColorButton) => {gtk::ColorButtonBuilder};
	    (ColorChooserDialog) => {gtk::ColorChooserDialogBuilder};
	    (ColorChooserWidget) => {gtk::ColorChooserWidgetBuilder};
	    (ComboBox) => {gtk::ComboBoxBuilder};
	    (ComboBoxText) => {gtk::ComboBoxTextBuilder};
	    (Dialog) => {gtk::DialogBuilder};
	    (DrawingArea) => {gtk::DrawingAreaBuilder};
	    (Entry) => {gtk::EntryBuilder};
	    (EntryCompletion) => {gtk::EntryCompletionBuilder};
	    (EventBox) => {gtk::EventBoxBuilder};
	    (Expander) => {gtk::ExpanderBuilder};
	    (FileChooserButton) => {gtk::FileChooserButtonBuilder};
	    (FileChooserDialog) => {gtk::FileChooserDialogBuilder};
	    (FileChooserNative) => {gtk::FileChooserNativeBuilder};
	    (FileChooserWidget) => {gtk::FileChooserWidgetBuilder};
	    (FlowBox) => {gtk::FlowBoxBuilder};
	    (FlowBoxChild) => {gtk::FlowBoxChildBuilder};
	    (FontButtonBuilder) => {gtk::FontButtonBuilderBuilder};
	    (FontChooserDialog) => {gtk::FontChooserDialogBuilder};
	    (FontChooserWidget) => {gtk::FontChooserWidgetBuilder};
	    (Frame) => {gtk::FrameBuilder};
	    (GLArea) => {gtk::GLAreaBuilder};
	    (GestureDrag) => {gtk::GestureDragBuilder};
	    (GestureLongPress) => {gtk::GestureLongPressBuilder};
	    (GestureMultiPress) => {gtk::GestureMultiPressBuilder};
	    (GesturePan) => {gtk::GesturePanBuilder};
	    (GestureRotate) => {gtk::GestureRotateBuilder};
	    (GestureSwipe) => {gtk::GestureSwipeBuilder};
	    (GestureZoom) => {gtk::GestureZoomBuilder};
	    (Grid) => {gtk::GridBuilder};
	    (HeaderBar) => {gtk::HeaderBarBuilder};
	    (IMContextSimple) => {gtk::IMContextSimpleBuilder};
	    (IMMulticontext) => {gtk::IMMulticontextBuilder};
	    (Image) => {gtk::ImageBuilder};
	    (InfoBar) => {gtk::InfoBarBuilder};
	    (Invisible) => {gtk::InvisibleBuilder};
	    (Label) => {gtk::LabelBuilder};
	    (Layout) => {gtk::LayoutBuilder};
	    (LevelBar) => {gtk::LevelBarBuilder};
	    (LinkButton) => {gtk::LinkButtonBuilder};
	    (ListBox) => {gtk::ListBoxBuilder};
	    (ListBoxRow) => {gtk::ListBoxRowBuilder};
	    (LockButton) => {gtk::LockButtonBuilder};
	    (MenuBar) => {gtk::MenuBarBuilder};
	    (MenuButton) => {gtk::MenuButtonBuilder};
	    (MenuItem) => {gtk::MenuItemBuilder};
	    (MenuToolButton) => {gtk::MenuToolButtonBuilder};
	    (MessageDialog) => {gtk::MessageDialogBuilder};
	    (ModelButton) => {gtk::ModelButtonBuilder};
	    (MountOperation) => {gtk::MountOperationBuilder};
	    (Notenook) => {gtk::NotenookBuilder};
	    (OffscreenWindow) => {gtk::OffscreenWindowBuilder};
	    (Overlay) => {gtk::OverlayBuilder};
	    (PadController) => {gtk::PadControllerBuilder};
	    (Paned) => {gtk::PanedBuilder};
	    (PlacesSidebar) => {gtk::PlacesSidebarBuilder};
	    (Plug) => {gtk::PlugBuilder};
	    (Popover) => {gtk::PopoverBuilder};
	    (PopoverMenu) => {gtk::PopoverMenuBuilder};
	    (PrintOperation) => {gtk::PrintOperationBuilder};
	    (ProgressBar) => {gtk::ProgressBarBuilder};
	    (RadioButton) => {gtk::RadioButtonBuilder};
	    (RadioMenuItem) => {gtk::RadioMenuItemBuilder};
	    (RadioToolButton) => {gtk::RadioToolButtonBuilder};
	    (RecentChooserDialog) => {gtk::RecentChooserDialogBuilder};
	    (RecentChooserMenu) => {gtk::RecentChooserMenuBuilder};
	    (RecentChooserWidget) => {gtk::RecentChooserWidgetBuilder};
	    (RecentManager) => {gtk::RecentManagerBuilder};
	    (Revealer) => {gtk::RevealerBuilder};
	    (Scale) => {gtk::ScaleBuilder};
	    (ScaleButton) => {gtk::ScaleButtonBuilder};
	    (Scrollbar) => {gtk::ScrollbarBuilder};
	    (ScrolledWindow) => {gtk::ScrolledWindowBuilder};
	    (SearchBar) => {gtk::SearchBarBuilder};
	    (SearchEntry) => {gtk::SearchEntryBuilder};
	    (Separator) => {gtk::SeparatorBuilder};
	    (SeparatorMenuItem) => {gtk::SeparatorMenuItemBuilder};
	    (SeparatorToolItem) => {gtk::SeparatorToolItemBuilder};
	    (SizeGroup) => {gtk::SizeGroupBuilder};
	    (Socket) => {gtk::SocketBuilder};
	    (SpinButton) => {gtk::SpinButtonBuilder};
	    (Spinner) => {gtk::SpinnerBuilder};
	    (Stack) => {gtk::StackBuilder};
	    (StackSidebar) => {gtk::BuilderStackSidebar};
	    (StackSwitcher) => {gtk::StackSwitcherBuilder};
	    (Statusbar) => {gtk::StatusbarBuilder};
	    (StyleContext) => {gtk::StyleContextBuilder};
	    (Switch) => {gtk::SwitchBuilder};
	    (TextBuffer) => {gtk::TextBufferBuilder};
	    (TextTag) => {gtk::TextTagBuilder};
	    (TextView) => {gtk::TextViewBuilder};
	    (ToggleButton) => {gtk::ToggleButtonBuilder};
	    (ToggleToolButton) => {gtk::ToggleToolButtonBuilder};
	    (ToolButton) => {gtk::ToolButtonBuilder};
	    (ToolItem) => {gtk::ToolItemBuilder};
	    (ToolItemGroup) => {gtk::ToolItemGroupBuilder};
	    (ToolPalette) => {gtk::ToolPaletteBuilder};
	    (Toolbar) => {gtk::ToolbarBuilder};
	    (TreeView) => {gtk::TreeViewBuilder};
	    (TreeViewColumn) => {gtk::TreeViewColumnBuilder};
	    (Viewport) => {gtk::ViewportBuilder};
	    (VolumeButton) => {gtk::VolumeButtonBuilder};
	    (Window) => {gtk::WindowBuilder};
	}

	// A helper macro to find Builders for Widgets etc.
	#[doc(hidden)]
	#[macro_export]
	macro_rules! __grade_find_builder {
	    ($n:ident:$p:ident) => {$crate::__grade_find_builder_internal!($p)};
	    ($p:ident) => {$crate::__grade_find_builder_internal!($p)};
	}

	// A helper macro to make parent widgets identifiable
	#[doc(hidden)]
	#[macro_export]
	macro_rules! __grade_link_to_name {
	   ($n:ident:$p:ident, $parent:ident) => {let $n = & $parent;};
	   ($p:ident, $parent:ident) => {};
	}

	#[macro_export]
	macro_rules! __build_func {
	    ($parent:ident, [$pack_func:ident $(,$pack_arg:expr)*], $child:ident$(:$child_alias:ident)? {$($child_body:tt)*}) => {
	        $parent.$pack_func(
	            & $crate::build! {$child$(:$child_alias)? {$($child_body)*}}.upcast::<gtk::Widget>(),
	                $($pack_arg),*
	        );
	    };
	    ($parent:ident, $child:ident$(:$child_alias:ident)? {$($child_body:tt)*}) => {
	        $parent.add(
	            & build! { $child$(:$child_alias)? { $($child_body)* } }.upcast::<gtk::Widget>()
	        );
	    };
	}

	// A helper macro to connect signals in widgets
	#[doc(hidden)]
	#[macro_export]
	macro_rules! __grade_connect_signal {
	    ($grade_widget:ident, notify($field:ident), $callback:expr) => { let _ = $grade_widget.connect_notify(Some("$field"), $callback); }; 
	    ($grade_widget:ident, $signal:path, $callback:expr) => { let _ =$grade_widget.connect("$signal", true, $callback);};
	}

	/// Instantiate a buildable Object from an inline macro
	#[macro_export]
	macro_rules! build {
	    { 
	        $parent:ident$(:$parent_alias:ident)? {
	            $( $field:ident:$val:expr, )*
	            $( => $signal:path: $callback:expr, )*
	            $( -- $([$pack_func:ident $(,$pack_arg:expr)*])? $child:ident$(:$child_alias:ident)? {$($child_body:tt)*})*
	            $( .. $meth:ident($($meth_body:expr),*) )*
	        } 
	    } => {
	        {

	           #[allow(unused_mut)] let mut builder = <$crate::__grade_find_builder!($parent$(:$parent_alias)?)>::new();
	           $(builder = builder.$field($val); )*
	           let grade_widget = builder.build();
	           $crate::__grade_link_to_name!($parent$(:$parent_alias)?, grade_widget);
	           $( $crate::__build_func!(grade_widget, $([$pack_func $(,$pack_arg)*],)? $child$(:$child_alias)? {$($child_body)*}); )*
	           $( grade_widget.$meth($($meth_body),*); )*
	           $( $crate::__grade_connect_signal!(grade_widget, $signal, $callback);)*
	           grade_widget
	        }
	    };
	}
}


#[cfg(test)]
mod tests {
	use gtk::*;
	use pango::EllipsizeMode;
	use crate::build;

	macro_rules! assert_gtype {
		($x:expr, $typ:ty) => (assert!($x.clone().dynamic_cast::<$typ>().is_ok()););
	}


    #[test]
    fn inline_application() {
    	let app = Application::new(Some("com.github.grade.test.inline_application"), Default::default()).unwrap();

        let grade_window = build! {
            ApplicationWindow {
                application: &app,
                show_menubar: false,
                default_height: 300,
                default_width: 400,

                 => show_menubar_notify: |_win| {None},


                -- Viewport {
                	-- Grid {
                		hexpand: true,
                		vexpand: true,

                		-- [attach, 1, 3, 1, 1 ] _left_button:Button {
                			vexpand: true,
                			label: "Left",
                			name: "label_left",
                		}

                		-- [attach, 3, 3, 1, 1 ] _right_button:Button {
                			label: "Right",
                			name: "label_right",
                		}

                		-- [attach, 2, 3, 1, 1 ] Label {
                			label: "This is a Label, amazing!",
                			ellipsize: EllipsizeMode::End,

                		}

                		..attach(
                			& build! {
                				TextView {
                					buffer: & build! {
                						TextBuffer {
                							text: "This is a textview, look at it and despair!",
                						}
                					}, 
                				}
                			}.upcast::<Widget>(), 1+0, 1, 2+1, 1+1
                		)
                	}
                }

                ..set_titlebar( Some( &build! {
                	HeaderBar {
                		title: "A grade test app",

                		-- [pack_start] Button {
                			label: "Start",
                		}

                		-- [pack_end] _end_button:Button {
                			label: "End",
                		}
                	}
                }.upcast::<Widget>()))
            }
        };

        assert_gtype!(&grade_window, ApplicationWindow);
        
        let hbar = (&grade_window.get_titlebar().unwrap()).clone();
        let grid = &grade_window.get_child().unwrap().downcast::<Viewport>().unwrap().get_child().unwrap().downcast::<Grid>().unwrap().clone();
        let left_button = &grid.get_child_at(1,3).unwrap().downcast::<Button>().unwrap().clone();
        let mid_label = &grid.get_child_at(2,3).unwrap().downcast::<Label>().unwrap().clone();
        let right_button = &grid.get_child_at(3,3).unwrap().downcast::<Button>().unwrap().clone();
        let tv = &grid.get_child_at(1,1).unwrap().downcast::<TextView>().unwrap().clone();
        let (start_iter, end_iter) = &tv.get_buffer().unwrap().get_bounds();

        assert_eq!(hbar.clone().downcast::<HeaderBar>().unwrap().get_children().len(), 2);
        assert_eq!(hbar.clone().downcast::<HeaderBar>().unwrap().get_children()[0].clone().downcast::<Button>().unwrap().get_label().unwrap(), "Start");
        assert_eq!(hbar.clone().downcast::<HeaderBar>().unwrap().get_children()[1].clone().downcast::<Button>().unwrap().get_label().unwrap(), "End");
        assert_eq!(grid.get_children().len(), 4);
        println!("{:?}", &grid);
        assert_eq!(left_button.get_label().unwrap(),"Left");
        assert_eq!(WidgetExt::get_name(left_button).unwrap(),"label_left");
        assert_eq!(right_button.get_label().unwrap(),"Right");
        assert_eq!(WidgetExt::get_name(right_button).unwrap(),"label_right");
        assert_eq!(mid_label.get_label().unwrap(), "This is a Label, amazing!");
        assert_eq!(mid_label.get_ellipsize(), EllipsizeMode::End);
        assert_eq!(grid.get_cell_height(&left_button.clone()), 1);
        assert_eq!(grid.get_cell_height(&right_button.clone()), 1);
        assert_eq!(grid.get_cell_height(&mid_label.clone()), 1);
        assert_eq!(grid.get_cell_height(&tv.clone()), 2);
        assert_eq!(grid.get_cell_width(&left_button.clone()), 1);
        assert_eq!(grid.get_cell_width(&right_button.clone()), 1);
        assert_eq!(grid.get_cell_width(&mid_label.clone()), 1);
        assert_eq!(grid.get_cell_width(&tv.clone()), 3);
        assert_eq!(tv.get_buffer().unwrap().get_text(start_iter, end_iter, true).unwrap(), "This is a textview, look at it and despair!");

    }
}

