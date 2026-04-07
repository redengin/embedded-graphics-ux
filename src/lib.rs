#![cfg_attr(not(feature = "std"), no_std)]

// /// Manages ordered entering and exiting of pages
// /// As guidance, good UX should use a DEPTH <= 3
// // pub struct NavigationStack<'a, const DEPTH: usize, P>
// pub struct NavigationStack<'a, const DEPTH: usize>
// {
//     page_stack: [Option<&'a dyn Page>; DEPTH],
//     current_depth: usize,
// }

// impl<'a, const DEPTH: usize, P> NavigationStack<'a, DEPTH, P>
//     where P: Page + core::marker::Copy,
// {
//     // pub fn new(root: &P) -> Self
//     pub fn new() -> Self
//     {
//         let mut page_stack: [Option<&P>; DEPTH] = [None; DEPTH];
//         // page_stack[0] = Some(&root);

//         Self {
//             current_depth: 0,
//             page_stack
//         }
//     }
//
//     // /// return toward root
//     // pub fn pop(&mut self) {
//     //     if self.current_depth > 0 {
//     //         self.current_depth = self.current_depth - 1;
//     //     }
//     // }
//
//     // /// add a subpage to the stack
//     // pub fn push(&mut self, page: P) {
//     //     if self.current_depth < DEPTH {
//     //         self.current_depth = self.current_depth + 1;
//     //         self.page_stack[self.current_depth] = Some(page);
//     //     }
//     //     else {
//     //         panic!("Navigation depth exceeded [max depth:{DEPTH}]");
//     //     }        
//     // }
// }

// /// Controls the full display
// pub trait Page {
//     /// repaint the whole screen
//     fn refresh(&mut self, display: &mut impl DrawTarget<Color = Rgb888>);

//     // /// handle HidEvent
//     // /// # Returns
//     // ///     * None - stay on this page
//     // ///     * Exit - return to the parent page 
//     // ///     * Enter(Page) - enter into a subpage
//     // // fn handle_event(&mut self, event: HidEvent) -> Option<NavigationEvent<Box<Self>>>;
//     // fn handle_event(&mut self, event: HidEvent) -> Option<NavigationEvent>;

//     // /// update the display
//     // /// * only needs to update changed items
//     // fn update<D:DrawTarget>(&mut self, display: &mut D)
//     // where
//     //     <D as DrawTarget>::Color: From<Rgb888>,
//     // ;
// }

// pub enum NavigationEvent {
//     /// return to the parent page
//     Exit,
//     // /// enter into a subpage
//     // Enter(Box<&dyn Page>),
// }

// pub enum HidEvent {
//     /// move to next selectable item
//     Next,
//     /// move to the previous selectable item
//     Previous,
//     /// invokes the selected item's handler
//     Select,
//     /// finds the touched item and invokes a 'Select' event
//     Touch { x: usize, y: usize },
// }


// // pub struct Ux<'a, const NAV_DEPTH: usize, P>
// pub struct Ux<'a, const NAV_DEPTH: usize>
//     // where
//     //     P : Page + core::marker::Copy,
// {
//     nav_stack: NavigationStack<'a, NAV_DEPTH>,

//     // root: P,
// }

// impl<'a, const NAV_DEPTH: usize, P> Ux<'a, NAV_DEPTH, P>
//     where
//         P : Page + core::marker::Copy,
// {
//     pub fn new() -> Self {
//         let root = DummyPage {};
//         Self {
//             // nav_stack: NavigationStack::new(&'a root),
//             nav_stack: NavigationStack::new()
//             // root,
//         }
//     }
//     // /// repaint the whole screen
//     // pub fn refresh<Display, Color>(&mut self, drawtarget: &mut Display)
//     // where
//     //     Display: DrawTarget<Color = Color>,
//     //     Color: PixelColor + Into<Rgb888> + From<Rgb888>,
//     // {
//     //     let mut display = Self::to_display(drawtarget);
//     //     // TODO update screen
//     //     let _ = display.clear(Rgb888::WHITE);
//     // }
//     // /// update the display
//     // /// * only updates changed items
//     // ///     * HidEvent
//     // ///     * Animations
//     // pub fn update<Display, Color>(&mut self, drawtarget: &mut Display)
//     // where
//     //     Display: DrawTarget<Color = Color>,
//     //     Color: PixelColor + Into<Rgb888> + From<Rgb888>,
//     // {
//     //     let mut display = Self::to_display(drawtarget);
//     //     // TODO update screen
//     //     let _ = display.clear(Rgb888::WHITE);
//     // }
//     // /// handle HidEvent
//     // /// @post call update() to show screen changes
//     // pub fn handle_event(&mut self, event: HidEvent) {
//     //     match event {
//     //         // TODO implement
//     //         _ => {}
//     //     }
//     // }

//     fn to_display<Display, Color>(drawtarget: &mut Display) -> ColorConverted<'_, Display, Rgb888>
//     where
//         Display: DrawTarget<Color = Color>,
//         Color: PixelColor + Into<Rgb888> + From<Rgb888>,
//     {
//         // transmute the drawtarget to an RGB color scheme
//         return drawtarget.color_converted();
//     }
// }

// pub struct DummyPage {
//     pub changed: bool,
// }

// impl DummyPage {
//     pub fn new() -> Self {
//         Self {
//             changed: false
//         }
//     }
// }
// impl Page for DummyPage {
//     fn refresh<Display:DrawTarget>(&mut self, display: &mut Display)
//     where
//         <Display as DrawTarget>::Color: From<Rgb888>,
//     {
//         let _ = display.clear(Rgb888::WHITE.into());
//     }

//     // fn handle_event(&mut self, _event: HidEvent) -> Option<NavigationEvent<dyn P>>
//     // {
//     //     let dummy_page = DummyPage::new();
//     //     return Some(NavigationEvent::Enter(dummy_page));
//     // }

//     // fn update<D:DrawTarget>(&mut self, display: &mut D)
//     // where
//     //     <D as DrawTarget>::Color: From<Rgb888>,
//     // {
//     //     if self.changed {
//     //         let _ = display.clear(Rgb888::WHITE.into());
//     //     }
//     //     else {
//     //         let _ = display.clear(Rgb888::BLACK.into());
//     //     }
//     //     self.changed = !self.changed;
//     // }
// }