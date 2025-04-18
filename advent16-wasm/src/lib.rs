pub mod advent16;
use advent16::{Kind, Map, Vrd, d};
use core::f64;
use itertools::Itertools;
use js_sys::Array;
use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::Rc,
    sync::{LazyLock, Mutex},
};
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, ResizeObserver, ResizeObserverEntry,
    ResizeObserverSize, Window, console, window,
};
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(start)]
fn init() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    draw();
    setup_resizer();
    start_animation();
    log!("init done");
    Ok(())
}
fn get_window_and_document() -> (Window, Document) {
    let window = window().expect("window");
    let document = window.document().expect("document");
    (window, document)
}
fn get_canvas_and_ctx() -> (HtmlCanvasElement, CanvasRenderingContext2d) {
    let (_, d) = get_window_and_document();
    let canvas = d
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let c = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    (canvas, c)
}
fn setup_resizer() {
    let closure = Closure::<dyn FnMut(_)>::new({
        let (canvas, _) = get_canvas_and_ctx();
        move |a: Array| {
            for e in a {
                let e = e
                    .dyn_into::<ResizeObserverEntry>()
                    .expect("ResizeObserverEntry");
                for s in e.content_box_size() {
                    let s = s
                        .dyn_into::<ResizeObserverSize>()
                        .expect("ResizeObserverSize");
                    canvas.set_height(num_traits::cast(s.block_size()).unwrap_or(100));
                    canvas.set_width(num_traits::cast(s.inline_size()).unwrap_or(100));
                    draw();
                }
            }
        }
    });
    let (canvas, _) = get_canvas_and_ctx();
    ResizeObserver::new(closure.as_ref().unchecked_ref())
        .expect("ResizeObserver")
        .observe(&canvas);
    closure.forget();
}

struct GlobalData {
    map: Map,
    rs: Option<Vrd>,
    start: f64,
    frame: usize,
}
static GLOBAL: LazyLock<Mutex<GlobalData>> = LazyLock::new(|| {
    Mutex::new(GlobalData {
        map: Map::new(d()),
        rs: Some(VecDeque::new()),
        start: 0.,
        frame: 0,
    })
});
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    let (w, _) = get_window_and_document();
    w.request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requestAnimationFrame");
}
fn get_now() -> f64 {
    let (w, _) = get_window_and_document();
    w.performance().expect("performance").now()
}
fn start_animation() {
    let inner_fn = Rc::new(RefCell::new(None));
    let outer_fn = inner_fn.clone();
    *outer_fn.borrow_mut() = Some(Closure::new(move || {
        let mut req_af = true;
        let mut g = GLOBAL.lock().expect("closure -> GLOBAL.lock()");
        const FPS: f64 = 60.;
        const MS_P_F: f64 = 1000. / FPS;
        let sf = g.frame;
        let ef = ((get_now() - g.start) / MS_P_F) as usize;
        let c = ef - sf;
        if c > usize::MAX - 1 {
            log!("skipping {}", (sf..ef).take(c - 1).join(","));
        }
        for _ in sf..ef {
            let rs = g.rs.take().expect("loop take Vrd");
            let (rs, done) = match g.map.step_once(rs) {
                Err(rs) => (rs, false),
                Ok(rs) => (rs, true),
            };
            g.rs = Some(rs);
            if done {
                req_af = false;
                log!(
                    "answer: {}",
                    g.map.best_seats(g.rs.as_ref().expect("best_seats"))
                );
                break;
            }
        }
        g.frame = ef;
        drop(g);
        if c > 0 {
            draw();
        }
        if req_af {
            request_animation_frame(inner_fn.borrow().as_ref().expect("inner_fn"));
        } else {
            let _ = inner_fn.borrow_mut().take();
        }
    }));
    GLOBAL
        .lock()
        .expect("start_animation -> GLOBAL.lock()")
        .start = get_now();
    request_animation_frame(outer_fn.borrow().as_ref().expect("outer_fn"));
}
fn draw() {
    let g = GLOBAL.lock().expect("draw -> GLOBAL.lock()");
    let (canvas, c) = get_canvas_and_ctx();
    let space_height = canvas.height() as f64 / g.map.m.len() as f64;
    let space_width = canvas.width() as f64 / g.map.m[0].len() as f64;
    for (y, r) in g.map.m.iter().enumerate() {
        for (x, sp) in r.iter().enumerate() {
            let mut color = match sp.k {
                Kind::Empty => "black",
                Kind::Wall => "blue",
                Kind::Start => "white",
                Kind::End => "#0F0",
            };
            // traveled
            if (1..usize::MAX).contains(&sp.c) && !matches!(sp.k, Kind::End) {
                color = "#338";
            }
            c.set_fill_style_str(color);
            c.fill_rect(
                x as f64 * space_width,
                y as f64 * space_height,
                space_width,
                space_height,
            );
        }
    }
    // reindeer
    let space_half_width = space_width / 2.;
    let space_half_height = space_height / 2.;
    c.set_fill_style_str("#F00");
    for (r, _) in g.rs.as_ref().expect("draw rs") {
        if r.p == g.map.e {
            continue;
        }
        let xl = r.p.x as f64 * space_width;
        let xm = xl + space_half_width;
        let xr = xl + space_width;
        let yt = r.p.y as f64 * space_height;
        let ym = yt + space_half_height;
        let yb = yt + space_height;
        c.begin_path();
        c.move_to(xl, ym);
        c.line_to(xm, yt);
        c.line_to(xr, ym);
        c.line_to(xm, yb);
        c.fill();
    }
}
