use eframe::egui;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::num::NonZeroU32;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

pub fn launch_gui() {
    let start = std::time::Instant::now();
    let gui_ready = Arc::new(AtomicBool::new(false));

    // Spawn the splash on its own thread so it doesn't block eframe's event loop.
    let splash_flag = gui_ready.clone();
    let splash_handle = std::thread::spawn(move || {
        run_splash(splash_flag);
    });

    let native_options = eframe::NativeOptions::default();
    let ready_flag = gui_ready.clone();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(move |cc| {
            let elapsed = start.elapsed();
            if let Ok(mut file) = std::fs::File::create("startup_time.txt") {
                let _ = writeln!(file, "Time to first frame setup: {:?}", elapsed);
            }
            Ok(Box::new(MyEguiApp::new(cc, ready_flag)))
        }),
    ).ok();

    let _ = splash_handle.join();
}

#[derive(Default)]
struct MyEguiApp {
    gui_ready: Option<Arc<AtomicBool>>,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>, gui_ready: Arc<AtomicBool>) -> Self {
        Self { gui_ready: Some(gui_ready) }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        static FIRST: std::sync::Once = std::sync::Once::new();
        if let Some(flag) = &self.gui_ready {
            FIRST.call_once(|| {
                flag.store(true, Ordering::SeqCst); // tells splash thread to close
                if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("startup_time.txt") {
                    let _ = writeln!(file, "First update() call happened");
                }
            });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}

// --- Splash screen: plain winit + softbuffer, no wgpu/glow shader init ---

struct SplashApp {
    window: Option<Arc<Window>>,
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
    ready_flag: Arc<AtomicBool>,
}

impl ApplicationHandler for SplashApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title("Loading...")
            .with_inner_size(winit::dpi::LogicalSize::new(280.0, 120.0))
            .with_resizable(false);
        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        let size = window.inner_size();
        surface.resize(
            NonZeroU32::new(size.width.max(1)).unwrap(),
            NonZeroU32::new(size.height.max(1)).unwrap(),
        ).unwrap();

        // Paint a plain grey background once.
        let mut buffer = surface.buffer_mut().unwrap();
        buffer.fill(0xFF2B2B2B);
        buffer.present().unwrap();

        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.ready_flag.load(Ordering::SeqCst) {
            event_loop.exit(); // real GUI is up, close splash
        }
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}

fn run_splash(ready_flag: Arc<AtomicBool>) {
    let event_loop = EventLoop::new().unwrap();
    let mut app = SplashApp { window: None, surface: None, ready_flag };
    let _ = event_loop.run_app(&mut app);
}