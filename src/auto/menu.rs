// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use AccelGroup;
use Align;
use Buildable;
use Container;
use LayoutManager;
use MenuShell;
use Overflow;
use ScrollType;
use Widget;
use gdk;
use gio;
use glib;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Menu(Object<gtk_sys::GtkMenu, gtk_sys::GtkMenuClass, MenuClass>) @extends MenuShell, Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_menu_get_type(),
    }
}

impl Menu {
    pub fn new() -> Menu {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_menu_new()).unsafe_cast()
        }
    }

    pub fn new_from_model<P: IsA<gio::MenuModel>>(model: &P) -> Menu {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_menu_new_from_model(model.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn get_for_attach_widget<P: IsA<Widget>>(widget: &P) -> Vec<Widget> {
        skip_assert_initialized!();
        unsafe {
            FromGlibPtrContainer::from_glib_none(gtk_sys::gtk_menu_get_for_attach_widget(widget.as_ref().to_glib_none().0))
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MenuBuilder {
    accel_group: Option<AccelGroup>,
    accel_path: Option<String>,
    active: Option<i32>,
    anchor_hints: Option<gdk::AnchorHints>,
    attach_widget: Option<Widget>,
    menu_type_hint: Option<gdk::SurfaceTypeHint>,
    monitor: Option<i32>,
    rect_anchor_dx: Option<i32>,
    rect_anchor_dy: Option<i32>,
    reserve_toggle_size: Option<bool>,
    take_focus: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    expand: Option<bool>,
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl MenuBuilder {
    pub fn new() -> Self {
        Self {
            accel_group: None,
            accel_path: None,
            active: None,
            anchor_hints: None,
            attach_widget: None,
            menu_type_hint: None,
            monitor: None,
            rect_anchor_dx: None,
            rect_anchor_dy: None,
            reserve_toggle_size: None,
            take_focus: None,
            can_focus: None,
            can_target: None,
            css_name: None,
            cursor: None,
            expand: None,
            focus_on_click: None,
            halign: None,
            has_focus: None,
            has_tooltip: None,
            height_request: None,
            hexpand: None,
            hexpand_set: None,
            is_focus: None,
            layout_manager: None,
            margin: None,
            margin_bottom: None,
            margin_end: None,
            margin_start: None,
            margin_top: None,
            name: None,
            opacity: None,
            overflow: None,
            receives_default: None,
            sensitive: None,
            tooltip_markup: None,
            tooltip_text: None,
            valign: None,
            vexpand: None,
            vexpand_set: None,
            visible: None,
            width_request: None,
        }
    }

    pub fn build(self) -> Menu {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref accel_group) = self.accel_group {
            properties.push(("accel-group", accel_group));
        }
        if let Some(ref accel_path) = self.accel_path {
            properties.push(("accel-path", accel_path));
        }
        if let Some(ref active) = self.active {
            properties.push(("active", active));
        }
        if let Some(ref anchor_hints) = self.anchor_hints {
            properties.push(("anchor-hints", anchor_hints));
        }
        if let Some(ref attach_widget) = self.attach_widget {
            properties.push(("attach-widget", attach_widget));
        }
        if let Some(ref menu_type_hint) = self.menu_type_hint {
            properties.push(("menu-type-hint", menu_type_hint));
        }
        if let Some(ref monitor) = self.monitor {
            properties.push(("monitor", monitor));
        }
        if let Some(ref rect_anchor_dx) = self.rect_anchor_dx {
            properties.push(("rect-anchor-dx", rect_anchor_dx));
        }
        if let Some(ref rect_anchor_dy) = self.rect_anchor_dy {
            properties.push(("rect-anchor-dy", rect_anchor_dy));
        }
        if let Some(ref reserve_toggle_size) = self.reserve_toggle_size {
            properties.push(("reserve-toggle-size", reserve_toggle_size));
        }
        if let Some(ref take_focus) = self.take_focus {
            properties.push(("take-focus", take_focus));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new(Menu::static_type(), &properties).expect("object new").downcast().expect("downcast")
    }

    pub fn accel_group(mut self, accel_group: &AccelGroup) -> Self {
        self.accel_group = Some(accel_group.clone());
        self
    }

    pub fn accel_path(mut self, accel_path: &str) -> Self {
        self.accel_path = Some(accel_path.to_string());
        self
    }

    pub fn active(mut self, active: i32) -> Self {
        self.active = Some(active);
        self
    }

    pub fn anchor_hints(mut self, anchor_hints: gdk::AnchorHints) -> Self {
        self.anchor_hints = Some(anchor_hints);
        self
    }

    pub fn attach_widget(mut self, attach_widget: &Widget) -> Self {
        self.attach_widget = Some(attach_widget.clone());
        self
    }

    pub fn menu_type_hint(mut self, menu_type_hint: gdk::SurfaceTypeHint) -> Self {
        self.menu_type_hint = Some(menu_type_hint);
        self
    }

    pub fn monitor(mut self, monitor: i32) -> Self {
        self.monitor = Some(monitor);
        self
    }

    pub fn rect_anchor_dx(mut self, rect_anchor_dx: i32) -> Self {
        self.rect_anchor_dx = Some(rect_anchor_dx);
        self
    }

    pub fn rect_anchor_dy(mut self, rect_anchor_dy: i32) -> Self {
        self.rect_anchor_dy = Some(rect_anchor_dy);
        self
    }

    pub fn reserve_toggle_size(mut self, reserve_toggle_size: bool) -> Self {
        self.reserve_toggle_size = Some(reserve_toggle_size);
        self
    }

    pub fn take_focus(mut self, take_focus: bool) -> Self {
        self.take_focus = Some(take_focus);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn layout_manager(mut self, layout_manager: &LayoutManager) -> Self {
        self.layout_manager = Some(layout_manager.clone());
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_MENU: Option<&Menu> = None;

pub trait MenuExt: 'static {
    //fn attach_to_widget<P: IsA<Widget>>(&self, attach_widget: &P, detacher: Option<Box<dyn FnOnce(&Widget, &Menu) + 'static>>);

    fn detach(&self);

    fn get_accel_group(&self) -> Option<AccelGroup>;

    fn get_accel_path(&self) -> Option<GString>;

    fn get_active(&self) -> Option<Widget>;

    fn get_attach_widget(&self) -> Option<Widget>;

    fn get_monitor(&self) -> i32;

    fn get_reserve_toggle_size(&self) -> bool;

    fn place_on_monitor(&self, monitor: &gdk::Monitor);

    fn popdown(&self);

    fn popup_at_pointer(&self, trigger_event: Option<&gdk::Event>);

    fn popup_at_rect<P: IsA<gdk::Surface>>(&self, rect_surface: &P, rect: &gdk::Rectangle, rect_anchor: gdk::Gravity, menu_anchor: gdk::Gravity, trigger_event: Option<&gdk::Event>);

    fn popup_at_widget<P: IsA<Widget>>(&self, widget: &P, widget_anchor: gdk::Gravity, menu_anchor: gdk::Gravity, trigger_event: Option<&gdk::Event>);

    fn reorder_child<P: IsA<Widget>>(&self, child: &P, position: i32);

    fn reposition(&self);

    fn set_accel_group<P: IsA<AccelGroup>>(&self, accel_group: Option<&P>);

    fn set_accel_path(&self, accel_path: Option<&str>);

    fn set_active(&self, index: u32);

    fn set_monitor(&self, monitor_num: i32);

    fn set_reserve_toggle_size(&self, reserve_toggle_size: bool);

    fn get_property_anchor_hints(&self) -> gdk::AnchorHints;

    fn set_property_anchor_hints(&self, anchor_hints: gdk::AnchorHints);

    fn set_property_attach_widget(&self, attach_widget: Option<&Widget>);

    fn get_property_menu_type_hint(&self) -> gdk::SurfaceTypeHint;

    fn set_property_menu_type_hint(&self, menu_type_hint: gdk::SurfaceTypeHint);

    fn get_property_rect_anchor_dx(&self) -> i32;

    fn set_property_rect_anchor_dx(&self, rect_anchor_dx: i32);

    fn get_property_rect_anchor_dy(&self) -> i32;

    fn set_property_rect_anchor_dy(&self, rect_anchor_dy: i32);

    fn connect_move_scroll<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_scroll(&self, scroll_type: ScrollType);

    //fn connect_popped_up<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_anchor_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_attach_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_menu_type_hint_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_monitor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rect_anchor_dx_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rect_anchor_dy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_reserve_toggle_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Menu>> MenuExt for O {
    //fn attach_to_widget<P: IsA<Widget>>(&self, attach_widget: &P, detacher: Option<Box<dyn FnOnce(&Widget, &Menu) + 'static>>) {
    //    unsafe { TODO: call gtk_sys:gtk_menu_attach_to_widget() }
    //}

    fn detach(&self) {
        unsafe {
            gtk_sys::gtk_menu_detach(self.as_ref().to_glib_none().0);
        }
    }

    fn get_accel_group(&self) -> Option<AccelGroup> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_get_accel_group(self.as_ref().to_glib_none().0))
        }
    }

    fn get_accel_path(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_get_accel_path(self.as_ref().to_glib_none().0))
        }
    }

    fn get_active(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_get_active(self.as_ref().to_glib_none().0))
        }
    }

    fn get_attach_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_get_attach_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn get_monitor(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_menu_get_monitor(self.as_ref().to_glib_none().0)
        }
    }

    fn get_reserve_toggle_size(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_menu_get_reserve_toggle_size(self.as_ref().to_glib_none().0))
        }
    }

    fn place_on_monitor(&self, monitor: &gdk::Monitor) {
        unsafe {
            gtk_sys::gtk_menu_place_on_monitor(self.as_ref().to_glib_none().0, monitor.to_glib_none().0);
        }
    }

    fn popdown(&self) {
        unsafe {
            gtk_sys::gtk_menu_popdown(self.as_ref().to_glib_none().0);
        }
    }

    fn popup_at_pointer(&self, trigger_event: Option<&gdk::Event>) {
        unsafe {
            gtk_sys::gtk_menu_popup_at_pointer(self.as_ref().to_glib_none().0, trigger_event.to_glib_none().0);
        }
    }

    fn popup_at_rect<P: IsA<gdk::Surface>>(&self, rect_surface: &P, rect: &gdk::Rectangle, rect_anchor: gdk::Gravity, menu_anchor: gdk::Gravity, trigger_event: Option<&gdk::Event>) {
        unsafe {
            gtk_sys::gtk_menu_popup_at_rect(self.as_ref().to_glib_none().0, rect_surface.as_ref().to_glib_none().0, rect.to_glib_none().0, rect_anchor.to_glib(), menu_anchor.to_glib(), trigger_event.to_glib_none().0);
        }
    }

    fn popup_at_widget<P: IsA<Widget>>(&self, widget: &P, widget_anchor: gdk::Gravity, menu_anchor: gdk::Gravity, trigger_event: Option<&gdk::Event>) {
        unsafe {
            gtk_sys::gtk_menu_popup_at_widget(self.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0, widget_anchor.to_glib(), menu_anchor.to_glib(), trigger_event.to_glib_none().0);
        }
    }

    fn reorder_child<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            gtk_sys::gtk_menu_reorder_child(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, position);
        }
    }

    fn reposition(&self) {
        unsafe {
            gtk_sys::gtk_menu_reposition(self.as_ref().to_glib_none().0);
        }
    }

    fn set_accel_group<P: IsA<AccelGroup>>(&self, accel_group: Option<&P>) {
        unsafe {
            gtk_sys::gtk_menu_set_accel_group(self.as_ref().to_glib_none().0, accel_group.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_accel_path(&self, accel_path: Option<&str>) {
        unsafe {
            gtk_sys::gtk_menu_set_accel_path(self.as_ref().to_glib_none().0, accel_path.to_glib_none().0);
        }
    }

    fn set_active(&self, index: u32) {
        unsafe {
            gtk_sys::gtk_menu_set_active(self.as_ref().to_glib_none().0, index);
        }
    }

    fn set_monitor(&self, monitor_num: i32) {
        unsafe {
            gtk_sys::gtk_menu_set_monitor(self.as_ref().to_glib_none().0, monitor_num);
        }
    }

    fn set_reserve_toggle_size(&self, reserve_toggle_size: bool) {
        unsafe {
            gtk_sys::gtk_menu_set_reserve_toggle_size(self.as_ref().to_glib_none().0, reserve_toggle_size.to_glib());
        }
    }

    fn get_property_anchor_hints(&self) -> gdk::AnchorHints {
        unsafe {
            let mut value = Value::from_type(<gdk::AnchorHints as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"anchor-hints\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_anchor_hints(&self, anchor_hints: gdk::AnchorHints) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"anchor-hints\0".as_ptr() as *const _, Value::from(&anchor_hints).to_glib_none().0);
        }
    }

    fn set_property_attach_widget(&self, attach_widget: Option<&Widget>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"attach-widget\0".as_ptr() as *const _, Value::from(attach_widget).to_glib_none().0);
        }
    }

    fn get_property_menu_type_hint(&self) -> gdk::SurfaceTypeHint {
        unsafe {
            let mut value = Value::from_type(<gdk::SurfaceTypeHint as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"menu-type-hint\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_menu_type_hint(&self, menu_type_hint: gdk::SurfaceTypeHint) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"menu-type-hint\0".as_ptr() as *const _, Value::from(&menu_type_hint).to_glib_none().0);
        }
    }

    fn get_property_rect_anchor_dx(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"rect-anchor-dx\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_rect_anchor_dx(&self, rect_anchor_dx: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"rect-anchor-dx\0".as_ptr() as *const _, Value::from(&rect_anchor_dx).to_glib_none().0);
        }
    }

    fn get_property_rect_anchor_dy(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"rect-anchor-dy\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_rect_anchor_dy(&self, rect_anchor_dy: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"rect-anchor-dy\0".as_ptr() as *const _, Value::from(&rect_anchor_dy).to_glib_none().0);
        }
    }

    fn connect_move_scroll<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"move-scroll\0".as_ptr() as *const _,
                Some(transmute(move_scroll_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_move_scroll(&self, scroll_type: ScrollType) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("move-scroll", &[&scroll_type]).unwrap() };
    }

    //fn connect_popped_up<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented flipped_rect: *.Pointer
    //    Unimplemented final_rect: *.Pointer
    //}

    fn connect_property_accel_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accel-group\0".as_ptr() as *const _,
                Some(transmute(notify_accel_group_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_accel_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accel-path\0".as_ptr() as *const _,
                Some(transmute(notify_accel_path_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::active\0".as_ptr() as *const _,
                Some(transmute(notify_active_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_anchor_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::anchor-hints\0".as_ptr() as *const _,
                Some(transmute(notify_anchor_hints_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_attach_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::attach-widget\0".as_ptr() as *const _,
                Some(transmute(notify_attach_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_menu_type_hint_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::menu-type-hint\0".as_ptr() as *const _,
                Some(transmute(notify_menu_type_hint_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_monitor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::monitor\0".as_ptr() as *const _,
                Some(transmute(notify_monitor_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_rect_anchor_dx_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rect-anchor-dx\0".as_ptr() as *const _,
                Some(transmute(notify_rect_anchor_dx_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_rect_anchor_dy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rect-anchor-dy\0".as_ptr() as *const _,
                Some(transmute(notify_rect_anchor_dy_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_reserve_toggle_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::reserve-toggle-size\0".as_ptr() as *const _,
                Some(transmute(notify_reserve_toggle_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn move_scroll_trampoline<P, F: Fn(&P, ScrollType) + 'static>(this: *mut gtk_sys::GtkMenu, scroll_type: gtk_sys::GtkScrollType, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast(), from_glib(scroll_type))
}

unsafe extern "C" fn notify_accel_group_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_accel_path_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_active_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_anchor_hints_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_attach_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_menu_type_hint_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_monitor_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rect_anchor_dx_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rect_anchor_dy_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_reserve_toggle_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Menu")
    }
}
