extern crate dd_gui;

use dd_gui::winit;
use dd_gui::glutin;
use dd_gui::glium::{ DisplayBuild };

use dd_gui::{ Point, Rect };
use dd_gui::widgets::{ Button, Knob };
use dd_gui::color;

fn main() {
    let wb = winit::WindowBuilder::new()
        .with_dimensions(640,480)
        .with_transparency(false);

    let display = glutin::WindowBuilder::from_winit_builder(wb)
        .with_decorations(true)
        .with_vsync()
        .build_glium()
        .unwrap();

    let mut renderer = dd_gui::Renderer::new(display);
    let mut ui = dd_gui::Ui::new(&mut renderer);

    let mut last_update = std::time::Instant::now();
    'main: loop {
        let now = std::time::Instant::now();
        let duration_since_last_update = now.duration_since(last_update);

        let sixteen_ms = std::time::Duration::from_millis(16);

        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update)
        } else {
            // Display FPS:
            // println!("FPS: {}", 1_000_000_000 / duration_since_last_update.subsec_nanos());

            let events: Vec<glutin::Event> = renderer.display.poll_events().collect();
            ui.handle_events(&events);

            Button::new(Rect{ origin: Point::new(100., 100.), size: Point::new(10., 100.) })
                .color(color::GREEN)
                .handle(&events, &mut ui, "green button".to_string())
                .draw(&mut renderer);

            Button::new(Rect{ origin: Point::new(130., 100.), size: Point::new(10., 100.) })
                .color(color::YELLOW)
                .handle(&events, &mut ui, "yellow button".to_string())
                .draw(&mut renderer);

            Button::new(Rect{ origin: Point::new(10., 10.), size: Point::new(50., 40.) })
                .draw(&mut renderer);

            if Knob::new(Rect{ origin: Point::new(20.,20.), size: Point::new(80.,50.) })
                .color(color::rgba(255, 200, 100, 150))
                .handle(&events, &mut ui, "default knob".to_string())
                .draw(&mut renderer)
                .changed() { println!("default knob!") };

//            if Knob::new(Rect{ origin: Point::new(150.,190.), size: Point::new(400.,400.) })
//                .color(color::PINK)
//                .handle(&events, &mut ui, "pink circle".to_string())
//                .draw(&mut renderer)
//                .changed() { println!("pink circle!"); };

            renderer.render();

            // break the loop on esc or window close.
            for event in events {
                match event {
                    glutin::Event::Closed => break 'main,
                    glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape)) => break 'main,
                    _ => ()
                }
            }

            last_update = now;
        }
    }
}
