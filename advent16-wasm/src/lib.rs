pub mod advent16;
use advent16::{Kind, Map, Vrd, d};
use core::f64;
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
        console::log_1(&format!( $( $t )* ).into())
    }
}

#[wasm_bindgen(start)]
fn init() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    draw().or_else(|| Some(log!("init -> draw failed")));
    setup_resizer().or_else(|| Some(log!("init -> setup_resizer failed")));
    start_animation().or_else(|| Some(log!("init -> start_animation failed")));
    log!("init done");
    Ok(())
}
fn get_window() -> Option<Window> {
    window()
}
fn get_document() -> Option<Document> {
    get_window()?.document()
}
fn get_canvas() -> Option<HtmlCanvasElement> {
    get_document()?
        .get_element_by_id("canvas")?
        .dyn_into::<HtmlCanvasElement>()
        .ok()
}
fn get_2d_ctx() -> Option<CanvasRenderingContext2d> {
    get_canvas()?
        .get_context("2d")
        .ok()??
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()
}
fn request_animation_frame(f: &Closure<dyn FnMut()>) -> Option<i32> {
    let w = get_window()?;
    w.request_animation_frame(f.as_ref().dyn_ref()?).ok()
}
fn get_now() -> Option<f64> {
    Some(get_window()?.performance()?.now())
}

fn setup_resizer() -> Option<()> {
    let closure = Closure::<dyn FnMut(_)>::new({
        move |a: Array| {
            (|| -> Option<()> {
                for e in a {
                    let e = e.dyn_into::<ResizeObserverEntry>().ok()?;
                    let canvas = e.target().dyn_into::<HtmlCanvasElement>().ok()?;
                    for s in e.content_box_size() {
                        let s = s.dyn_into::<ResizeObserverSize>().ok()?;
                        canvas.set_height(num_traits::cast(s.block_size()).unwrap_or(100));
                        canvas.set_width(num_traits::cast(s.inline_size()).unwrap_or(100));
                        draw()?;
                    }
                }
                Some(())
            })()
            .or_else(|| Some(log!("setup_resizer -> closure failed")));
        }
    });
    let canvas = get_canvas()?;
    let ro = ResizeObserver::new(closure.as_ref().dyn_ref()?).ok()?;
    ro.observe(&canvas);
    closure.forget();
    Some(())
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
fn start_animation() -> Option<()> {
    let inner_fn = Rc::new(RefCell::new(None));
    let outer_fn = inner_fn.clone();
    *outer_fn.borrow_mut() = Some(Closure::new(move || {
        (|| -> Option<()> {
            let mut req_af = true;
            let mut g = GLOBAL.lock().ok()?;
            const FPS: f64 = 60.;
            const MS_P_F: f64 = 1000. / FPS;
            let sf = g.frame;
            let ef = ((get_now()? - g.start) / MS_P_F) as usize;
            let c = ef - sf;
            for _ in sf..ef {
                let rs = g.rs.take()?;
                let res = g.map.step_once(rs);
                let done = res.is_ok();
                let rs = match res {
                    Err(rs) | Ok(rs) => rs,
                };
                g.rs.replace(rs);
                if done {
                    req_af = false;
                    let answer = g.map.best_seats(g.rs.as_ref()?);
                    log!("answer: {answer}",);
                    break;
                }
            }
            g.frame = ef;
            drop(g);
            if c > 0 {
                draw()?;
            }
            if req_af {
                request_animation_frame(inner_fn.borrow().as_ref()?)?;
            } else {
                let _ = inner_fn.borrow_mut().take();
            }
            Some(())
        })()
        .or_else(|| Some(log!("start_animation -> closure failed")));
    }));
    GLOBAL.lock().ok()?.start = get_now()?;
    request_animation_frame(outer_fn.borrow().as_ref()?)?;
    Some(())
}
fn draw() -> Option<()> {
    let g = GLOBAL.lock().ok()?;
    let canvas = get_canvas()?;
    let c = get_2d_ctx()?;
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
    for (r, _) in g.rs.as_ref()? {
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
    Some(())
}
