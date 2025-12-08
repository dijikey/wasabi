use crate::EventSystem;
use std::time::Instant;
use wasabi_traits::Layer;
use wasabi_traits::event::EventHandler;

#[test]
fn benchmark() {
    const N: usize = 8;
    const ALLOC_ITER: usize = 100_000;
    const BENCHMARK_ITER: usize = 100;

    #[derive(Debug)]
    struct Alpha([u128; N]);

    impl Default for Alpha {
        fn default() -> Self {
            Self([0; N])
        }
    }

    impl Layer for Alpha {
        fn render(&mut self) {
            println!("render");
        }
        fn update(&mut self) {
            println!("update");
        }
    }

    let mut event_sys = EventSystem::new();
    macro_rules! add {
        (event_sys: $sys:ident, tag_key: $tag:ident) => {
            $sys += Tag::$tag(Box::new(move || {
                let mut a = Alpha::default();
                a.0[0] += 1;
            }))
        };
    }

    let mut id = 0;
    for _ in 0..ALLOC_ITER {
        match id {
            0 => add!(event_sys: event_sys, tag_key: KeyPressed),
            1 => add!(event_sys: event_sys, tag_key: KeyReleased),
            2 => add!(event_sys: event_sys, tag_key: MousePressed),
            3 => add!(event_sys: event_sys, tag_key: MouseReleased),
            4 => add!(event_sys: event_sys, tag_key: MouseMoved),
            5 => add!(event_sys: event_sys, tag_key: WindowResized),
            6 => add!(event_sys: event_sys, tag_key: WindowClosed),
            7 => add!(event_sys: event_sys, tag_key: WindowFocus),
            8 => add!(event_sys: event_sys, tag_key: WindowLostFocus),
            9 => add!(event_sys: event_sys, tag_key: WindowMoved),
            _ => panic!("Invalid id"),
        }

        if id == 9 { id = 0 } else { id += 1 }
    }

    let now = Instant::now();
    for _ in 0..BENCHMARK_ITER {
        event_sys.key_pressed();
        event_sys.key_released();
        event_sys.mouse_pressed();
        event_sys.mouse_released();
        event_sys.mouse_moved();
        event_sys.window_moved();
        event_sys.window_closed();
        event_sys.window_resized();
        event_sys.window_focused();
        event_sys.window_lost_focus();
    }
    println!("============Benchmark============");
    println!("Elapsed time of call callbacks: {:?}", now.elapsed());
    println!("============MEMORY============");
    println!("Layer mem size: {:?}", size_of::<Alpha>());
    println!("Event system: {event_sys:?}");
    println!(
        "Event system mem size: Filled({}) & Struct({}) & Empty({})",
        size(&mut event_sys),
        size_of::<EventSystem>(),
        size_of_val(&EventSystem::default())
    );

    fn size(e: &mut EventSystem) -> usize {
        let mut result = 0;
        result += size_of::<EventSystem>();
        result += size_of_val(e.key_pressed_mut());
        result += size_of_val(e.key_released_mut());
        result += size_of_val(e.mouse_pressed_mut());
        result += size_of_val(e.mouse_released_mut());
        result += size_of_val(e.mouse_moved_mut());
        result += size_of_val(e.window_moved_mut());
        result += size_of_val(e.window_resized_mut());
        result += size_of_val(e.window_closed_mut());
        result += size_of_val(e.window_lost_focus_mut());
        result + size_of_val(e.window_focused_mut())
    }
}
