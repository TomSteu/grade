
pub mod macros {

	// A helper macro to init builders for Widgets etc.
	#[doc(hidden)]
	#[macro_export]
	macro_rules! __grade_builder_start {
	    (AboutDialog) => {gtk::AboutDialogBuilder::new()};
	    (AccelLabel) => {gtk::AccelLabelBuilder::new()};
	    (ActionBar) => {gtk::ActionBarBuilder::new()};
	    (AppChooserButton) => {gtk::AppChooserButtonBuilder::new()};
	    (AppChooserDialog) => {gtk::AppChooserDialogBuilder::new()};
	    (AppChooserWidget) => {gtk::AppChooserWidgetBuilder::new()};
	    (Application) => {gtk::ApplicationBuilder::new()};
	    (ApplicationWindow) => {gtk::ApplicationWindowBuilder::new()};
	    (AspectFrame) => {gtk::AspectFrameBuilder::new()};
	    (Assistant) => {gtk::AssistantBuilder::new()};
	    (Box) => {gtk::BoxBuilder::new()};
	    (Button) => {gtk::ButtonBuilder::new()};
	    (ButtonBox) => {gtk::ButtonBoxBuilder::new()};
	    (Calendear) => {gtk::CalendearBuilder::new()};
	    (CellAreaBox) => {gtk::CellAreaBoxBuilder::new()};
	    (CellRendererAccel) => {gtk::CellRendererAccelBuilder::new()};
	    (CellRendererCombo) => {gtk::CellRendererComboBuilder::new()};
	    (CellRendererPixbuf) => {gtk::CellRendererPixbufBuilder::new()};
	    (CellRendererProgress) => {gtk::CellRendererProgressBuilder::new()};
	    (CellRendererSpin) => {gtk::CellRendererSpinBuilder::new()};
	    (CellRendererSpinner) => {gtk::CellRendererSpinnerBuilder::new()};
	    (CellRendererText) => {gtk::CellRendererTextBuilder::new()};
	    (CellRendererToggle) => {gtk::CellRendererToggleBuilder::new()};
	    (CellView) => {gtk::CellViewBuilder::new()};
	    (CheckButton) => {gtk::CheckButtonBuilder::new()};
	    (CheckMenuItem) => {gtk::CheckMenuItemBuilder::new()};
	    (ColorButton) => {gtk::ColorButtonBuilder::new()};
	    (ColorChooserDialog) => {gtk::ColorChooserDialogBuilder::new()};
	    (ColorChooserWidget) => {gtk::ColorChooserWidgetBuilder::new()};
	    (ComboBox) => {gtk::ComboBoxBuilder::new()};
	    (ComboBoxText) => {gtk::ComboBoxTextBuilder::new()};
	    (Dialog) => {gtk::DialogBuilder::new()};
	    (DrawingArea) => {gtk::DrawingAreaBuilder::new()};
	    (Entry) => {gtk::EntryBuilder::new()};
	    (EntryCompletion) => {gtk::EntryCompletionBuilder::new()};
	    (EventBox) => {gtk::EventBoxBuilder::new()};
	    (Expander) => {gtk::ExpanderBuilder::new()};
	    (FileChooserButton) => {gtk::FileChooserButtonBuilder::new()};
	    (FileChooserDialog) => {gtk::FileChooserDialogBuilder::new()};
	    (FileChooserNative) => {gtk::FileChooserNativeBuilder::new()};
	    (FileChooserWidget) => {gtk::FileChooserWidgetBuilder::new()};
	    (FlowBox) => {gtk::FlowBoxBuilder::new()};
	    (FlowBoxChild) => {gtk::FlowBoxChildBuilder::new()};
	    (FontButtonBuilder) => {gtk::FontButtonBuilderBuilder::new()};
	    (FontChooserDialog) => {gtk::FontChooserDialogBuilder::new()};
	    (FontChooserWidget) => {gtk::FontChooserWidgetBuilder::new()};
	    (Frame) => {gtk::FrameBuilde::new()};
	    (GLArea) => {gtk::GLAreaBuilder::new()};
	    (GestureDrag) => {gtk::GestureDragBuilder::new()};
	    (GestureLongPress) => {gtk::GestureLongPressBuilder::new()};
	    (GestureMultiPress) => {gtk::GestureMultiPressBuilder::new()};
	    (GesturePan) => {gtk::GesturePanBuilder::new()};
	    (GestureRotate) => {gtk::GestureRotateBuilder::new()};
	    (GestureSwipe) => {gtk::GestureSwipeBuilder::new()};
	    (GestureZoom) => {gtk::GestureZoomBuilder::new()};
	    (Grid) => {gtk::GridBuilder::new()};
	    (HeaderBar) => {gtk::HeaderBarBuilder::new()};
	    (IMContextSimple) => {gtk::IMContextSimpleBuilder::new()};
	    (IMMulticontext) => {gtk::IMMulticontextBuilder::new()};
	    (Image) => {gtk::ImageBuilder::new()};
	    (InfoBar) => {gtk::InfoBarBuilder::new()};
	    (Invisible) => {gtk::InvisibleBuilder::new()};
	    (Label) => {gtk::LabelBuilder::new()};
	    (Layout) => {gtk::LayoutBuilder::new()};
	    (LevelBar) => {gtk::LevelBarBuilder::new()};
	    (LinkButton) => {gtk::LinkButtonBuilder::new()};
	    (ListBox) => {gtk::ListBoxBuilder::new()};
	    (ListBoxRow) => {gtk::ListBoxRowBuilder::new()};
	    (LockButton) => {gtk::LockButtonBuilder::new()};
	    (MenuBar) => {gtk::MenuBarBuilder::new()};
	    (MenuButton) => {gtk::MenuButtonBuilder::new()};
	    (MenuItem) => {gtk::MenuItemBuilder::new()};
	    (MenuToolButton) => {gtk::MenuToolButtonBuilder::new()};
	    (MessageDialog) => {gtk::MessageDialogBuilder::new()};
	    (ModelButton) => {gtk::ModelButtonBuilder::new()};
	    (MountOperation) => {gtk::MountOperationBuilder::new()};
	    (Notenook) => {gtk::NotenookBuilder::new()};
	    (OffscreenWindow) => {gtk::OffscreenWindowBuilder::new()};
	    (Overlay) => {gtk::OverlayBuilder::new()};
	    (PadController) => {gtk::PadControllerBuilder::new()};
	    (Paned) => {gtk::PanedBuilder::new()};
	    (PlacesSidebar) => {gtk::PlacesSidebarBuilder::new()};
	    (Plug) => {gtk::PlugBuilder::new()};
	    (Popover) => {gtk::PopoverBuilder::new()};
	    (PopoverMenu) => {gtk::PopoverMenuBuilder::new()};
	    (PrintOperation) => {gtk::PrintOperationBuilder::new()};
	    (ProgressBar) => {gtk::ProgressBarBuilder::new()};
	    (RadioButton) => {gtk::RadioButtonBuilder::new()};
	    (RadioMenuItem) => {gtk::RadioMenuItemBuilder::new()};
	    (RadioToolButton) => {gtk::RadioToolButtonBuilder::new()};
	    (RecentChooserDialog) => {gtk::RecentChooserDialogBuilder::new()};
	    (RecentChooserMenu) => {gtk::RecentChooserMenuBuilder::new()};
	    (RecentChooserWidget) => {gtk::RecentChooserWidgetBuilder::new()};
	    (RecentManager) => {gtk::RecentManagerBuilder::new()};
	    (Revealer) => {gtk::RevealerBuilder::new()};
	    (Scale) => {gtk::ScaleBuilder::new()};
	    (ScaleButton) => {gtk::ScaleButtonBuilder::new()};
	    (Scrollbar) => {gtk::ScrollbarBuilder::new()};
	    (ScrolledWindow) => {gtk::ScrolledWindowBuilder::new()};
	    (SearchBar) => {gtk::SearchBarBuilder::new()};
	    (SearchEntry) => {gtk::SearchEntryBuilder::new()};
	    (Separator) => {gtk::SeparatorBuilder::new()};
	    (SeparatorMenuItem) => {gtk::SeparatorMenuItemBuilder::new()};
	    (SeparatorToolItem) => {gtk::SeparatorToolItemBuilder::new()};
	    (SizeGroup) => {gtk::SizeGroupBuilder::new()};
	    (Socket) => {gtk::SocketBuilder::new()};
	    (SpinButton) => {gtk::SpinButtonBuilder::new()};
	    (Spinner) => {gtk::SpinnerBuilder::new()};
	    (Stack) => {gtk::StackBuilder::new()};
	    (StackSidebar) => {gtk::BuilderStackSidebar::new()};
	    (StackSwitcher) => {gtk::StackSwitcherBuilder::new()};
	    (Statusbar) => {gtk::StatusbarBuilder::new()};
	    (StyleContext) => {gtk::StyleContextBuilder::new()};
	    (Switch) => {gtk::SwitchBuilder::new()};
	    (TextBuffer) => {gtk::TextBufferBuilder::new()};
	    (TextTag) => {gtk::TextTagBuilder::new()};
	    (TextView) => {gtk::TextViewBuilder::new()};
	    (ToggleButton) => {gtk::ToggleButtonBuilder::new()};
	    (ToggleToolButton) => {gtk::ToggleToolButtonBuilder::new()};
	    (ToolButton) => {gtk::ToolButtonBuilder::new()};
	    (ToolItem) => {gtk::ToolItemBuilder::new()};
	    (ToolItemGroup) => {gtk::ToolItemGroupBuilder::new()};
	    (ToolPalette) => {gtk::ToolPaletteBuilder::new()};
	    (Toolbar) => {gtk::ToolbarBuilder::new()};
	    (TreeView) => {gtk::TreeViewBuilder::new()};
	    (TreeViewColumn) => {gtk::TreeViewColumnBuilder::new()};
	    (Viewport) => {gtk::ViewportBuilder::new()};
	    (VolumeButton) => {gtk::VolumeButtonBuilder::new()};
	    (Window) => {gtk::WindowBuilder::new()};
	    ($class:ty) => {<$class>::new()}; 
	}

	// A helper macro to finish builders for Widgets etc.
	#[doc(hidden)]
	#[macro_export]
	macro_rules! __grade_builder_end {
	    (AboutDialog, $builder:expr) => { $builder.build() };
	    (AccelLabel, $builder:expr) => { $builder.build() };
	    (ActionBar, $builder:expr) => { $builder.build() };
	    (AppChooserButton, $builder:expr) => { .$builderbuild() };
	    (AppChooserDialog, $builder:expr) => { $builder.build() };
	    (AppChooserWidget, $builder:expr) => { $builder.build() };
	    (Application, $builder:expr) => { $builder.build() };
	    (ApplicationWindow, $builder:expr) => { $builder.build() };
	    (AspectFrame, $builder:expr) => { $builder.build() };
	    (Assistant, $builder:expr) => { $builder.build() };
	    (Box, $builder:expr) => { $builder.build() };
	    (Button, $builder:expr) => { $builder.build() };
	    (ButtonBox, $builder:expr) => { $builder.build() };
	    (Calendear, $builder:expr) => { $builder.build() };
	    (CellAreaBox, $builder:expr) => { $builder.build() };
	    (CellRendererAccel, $builder:expr) => { $builder.build() };
	    (CellRendererCombo, $builder:expr) => { $builder.build() };
	    (CellRendererPixbuf, $builder:expr) => { $builder.build() };
	    (CellRendererProgress, $builder:expr) => { $builder.build() };
	    (CellRendererSpin, $builder:expr) => { $builder.build() };
	    (CellRendererSpinner, $builder:expr) => { $builder.build() };
	    (CellRendererText, $builder:expr) => { $builder.build() };
	    (CellRendererToggle, $builder:expr) => { $builder.build() };
	    (CellView, $builder:expr) => { $builder.build() };
	    (CheckButton, $builder:expr) => { $builder.build() };
	    (CheckMenuItem, $builder:expr) => { $builder.build() };
	    (ColorButton, $builder:expr) => { $builder.build() };
	    (ColorChooserDialog, $builder:expr) => { $builder.build() };
	    (ColorChooserWidget, $builder:expr) => { $builder.build() };
	    (ComboBox, $builder:expr) => { $builder.build() };
	    (ComboBoxText, $builder:expr) => { $builder.build() };
	    (Dialog, $builder:expr) => { $builder.build() };
	    (DrawingArea, $builder:expr) => { $builder.build() };
	    (Entry, $builder:expr) => { $builder.build() };
	    (EntryCompletion, $builder:expr) => { $builder.build() };
	    (EventBox, $builder:expr) => { $builder.build() };
	    (Expander, $builder:expr) => { $builder.build() };
	    (FileChooserButton, $builder:expr) => { $builder.build() };
	    (FileChooserDialog, $builder:expr) => { $builder.build() };
	    (FileChooserNative, $builder:expr) => { $builder.build() };
	    (FileChooserWidget, $builder:expr) => { $builder.build() };
	    (FlowBox, $builder:expr) => { $builder.build() };
	    (FlowBoxChild, $builder:expr) => { $builder.build() };
	    (FontButtonBuilder, $builder:expr) => { $builder.build() };
	    (FontChooserDialog, $builder:expr) => { $builder.build() };
	    (FontChooserWidget, $builder:expr) => { $builder.build() };
	    (Frame, $builder:expr) => { $builder.build() };
	    (GLArea, $builder:expr) => { $builder.build() };
	    (GestureDrag, $builder:expr) => { $builder.build() };
	    (GestureLongPress, $builder:expr) => { $builder.build() };
	    (GestureMultiPress, $builder:expr) => { $builder.build() };
	    (GesturePan, $builder:expr) => { $builder.build() };
	    (GestureRotate, $builder:expr) => { $builder.build() };
	    (GestureSwipe, $builder:expr) => { .$builderbuild() };
	    (GestureZoom, $builder:expr) =>{ $builder.build() };
	    (Grid, $builder:expr) => { $builder.build() };
	    (HeaderBar, $builder:expr) => { $builder.build() };
	    (IMContextSimple, $builder:expr) => { $builder.build() };
	    (IMMulticontext, $builder:expr) => { $builder.build() };
	    (Image, $builder:expr) => { $builderbuild() };
	    (InfoBar, $builder:expr) => { $builder.build() };
	    (Invisible, $builder:expr) =>{ $builder.build() };
	    (Label, $builder:expr) => { $builder.build() };
	    (Layout, $builder:expr) => { $builder.build() };
	    (LevelBar, $builder:expr) => { $builder.build() };
	    (LinkButton, $builder:expr) => { $builder.build() };
	    (ListBox, $builder:expr) => { $builder.build() };
	    (ListBoxRow, $builder:expr) => { $builder.build() };
	    (LockButton, $builder:expr) => { $builder.build() };
	    (MenuBar, $builder:expr) => { $builder.build() };
	    (MenuButton, $builder:expr) => { $builder.build() };
	    (MenuItem, $builder:expr) => { $builder.build() };
	    (MenuToolButton, $builder:expr) => { $builder.build() };
	    (MessageDialog, $builder:expr) => { $builder.build() };
	    (ModelButton, $builder:expr) => { $builder.build() };
	    (MountOperation, $builder:expr) => { $builder.build() };
	    (Notenook, $builder:expr) => { $builder.build() };
	    (OffscreenWindow, $builder:expr) => { $builder.build() };
	    (Overlay, $builder:expr) => { $builder.build() };
	    (PadController, $builder:expr) => { $builder.build() };
	    (Paned, $builder:expr) => { $builder.build() };
	    (PlacesSidebar, $builder:expr) => { $builder.build() };
	    (Plug, $builder:expr) => { $builder.build() };
	    (Popover, $builder:expr) => { $builder.build() };
	    (PopoverMenu, $builder:expr) => { $builder.build() };
	    (PrintOperation, $builder:expr) => { $builder.build() };
	    (ProgressBar, $builder:expr) => { $builder.build() };
	    (RadioButton, $builder:expr) => { $builder.build() };
	    (RadioMenuItem, $builder:expr) => { $builder.build() };
	    (RadioToolButton, $builder:expr) => { $builder.build() };
	    (RecentChooserDialog, $builder:expr) => { $builder.build() };
	    (RecentChooserMenu, $builder:expr) => { $builder.build() };
	    (RecentChooserWidget, $builder:expr) => { $builder.build() };
	    (RecentManager, $builder:expr) => { $builder.build() };
	    (Revealer, $builder:expr) => { $builder.build() };
	    (Scale, $builder:expr) => { $builder.build() };
	    (ScaleButton, $builder:expr) => { $builder.build() };
	    (Scrollbar, $builder:expr) => { $builder.build() };
	    (ScrolledWindow, $builder:expr) => { $builder.build() };
	    (SearchBar, $builder:expr) => { $builder.build() };
	    (SearchEntry, $builder:expr) => { .$builderbuild() };
	    (Separator, $builder:expr) => { $builder.build() };
	    (SeparatorMenuItem, $builder:expr) => { $builder.build() };
	    (SeparatorToolItem, $builder:expr) => { $builder.build() };
	    (SizeGroup, $builder:expr) =>{ $builder.build() };
	    (Socket, $builder:expr) => { $builder.build() };
	    (SpinButton, $builder:expr) => { $builder.build() };
	    (Spinner, $builder:expr) => { $builder.build() };
	    (Stack, $builder:expr) => { $builder.build() };
	    (StackSidebar, $builder:expr) =>{ $builder.build() };
	    (StackSwitcher, $builder:expr) => { $builder.build() };
	    (Statusbar, $builder:expr) => { $builder.build() };
	    (StyleContext, $builder:expr) => { $builder.build() };
	    (Switch, $builder:expr) => { $builder.build() };
	    (TextBuffer, $builder:expr) => { $builder.build() };
	    (TextTag, $builder:expr) => { $builder.build() };
	    (TextView, $builder:expr) => { $builder.build() };
	    (ToggleButton, $builder:expr) => { $builder.build() };
	    (ToggleToolButton, $builder:expr) => { $builder.build() };
	    (ToolButton, $builder:expr) => { $builder.build() };
	    (ToolItem, $builder:expr) => { $builder.build() };
	    (ToolItemGroup, $builder:expr) => { $builder.build() };
	    (ToolPalette, $builder:expr) => { $builder.build() };
	    (Toolbar, $builder:expr) => { $builder.build() };
	    (TreeView, $builder:expr) => { $builder.build() };
	    (TreeViewColumn, $builder:expr) => { $builder.build() };
	    (Viewport, $builder:expr) => { $builder.build() };
	    (VolumeButton, $builder:expr) => { $builder.build() };
	    (Window, $builder:expr) => { $builder.build() };
	    ($class:ty, $builder:expr) => {$builder };
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
	        $parent:ident:$parent_type:ident {
	            $( $field:ident:$val:expr, )*
	            $( => $signal:path: $callback:expr, )*
	            $( -- $([$pack_func:ident $(,$pack_arg:expr)*])? $child:ident$(:$child_alias:ident)? {$($child_body:tt)*})*
	            $( .. $meth:ident($($meth_body:expr),*)$(.$more_meth:ident( $($more_args:expr),* ))*  )*
	        } 
	    } => {
	        {
	           let $parent = $crate::__grade_builder_end!($parent_type,
	           	$crate::__grade_builder_start!($parent_type)
	           	$(.$field($val) )* 
	           );
	           $( $crate::__build_func!($parent, $([$pack_func $(,$pack_arg)*],)? $child$(:$child_alias)? {$($child_body)*}); )*
	           $( $parent.$meth($($meth_body),*)$(.$more_meth( $($more_args),* ))* ; )*
	           $( $crate::__grade_connect_signal!($parent, $signal, $callback);)*
	            $parent
	        }
	    };
	    { 
	        $parent:ident {
	            $( $field:ident:$val:expr, )*
	            $( => $signal:path: $callback:expr, )*
	            $( -- $([$pack_func:ident $(,$pack_arg:expr)*])? $child:ident$(:$child_alias:ident)? {$($child_body:tt)*})*
	            $( .. $meth:ident($($meth_body:expr),*)$(.$more_meth:ident( $($more_args:expr),* ))*  )*
	        } 
	    } => {
	        {

	           let grade_widget = $crate::__grade_builder_end!($parent,
	           	$crate::__grade_builder_start!($parent)
	           	$(.$field($val) )* 
	           );
	           $( $crate::__build_func!(grade_widget, $([$pack_func $(,$pack_arg)*],)? $child$(:$child_alias)? {$($child_body)*}); )*
	           $( grade_widget.$meth($($meth_body),*)$(.$more_meth( $($more_args),* ))* ; )*
	           $( $crate::__grade_connect_signal!(grade_widget, $signal, $callback);)*
	           grade_widget
	        }
	    };
	}

	/// Cascade style syntax
	#[macro_export]
	macro_rules! cascade {
		{
			$caller:ident {
				$(
					..$meth:ident($($arg:expr),*)$(.$more_meth:ident( $($more_args:expr),* ))* 
				)*
			}
		} => {
			$( $caller.$meth( $($arg),* )$(.$more_meth( $($more_args),* ))* ; )*
		};
	}
}


#[cfg(test)]
mod tests {
	use gtk::*;
	use pango::EllipsizeMode;
	use crate::{build, cascade};

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

    #[test]
    fn non_builder_create() {
    	let vec1 = Vec::<String>::new();
    	type Test = Vec::<String>;

    	let vec2 = build! {
    		Test {}
    	};

    	assert_eq!(vec1, vec2);
    }

    #[test]
    fn cascade_test1() {
    	let mut vec1 = Vec::<String>::new();
    	cascade! {
    		vec1 {
    			..push("1".into())
    			..push("2".into())
    			..push("3".into())
    		}
    	}; 

    	let vec2 = vec! ["1", "2", "3"];
    	assert_eq!(vec1, vec2);
    }

    #[test]
    fn cascade_test2() {
    	let test_object = Some(Some(1));
    	cascade! {
    		test_object {
    			..unwrap().unwrap()
    		}
    	} ;
    }
}

