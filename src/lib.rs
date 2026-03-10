use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Position, DeviceMotionEvent, HtmlElement};
use std::cell::RefCell;
use rhai::{Engine, Dynamic};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Pin {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub title: String,
}

thread_local! {
    static MAP_INSTANCE: RefCell<Option<JsValue>> = RefCell::new(None);
    static PIN_LAYER_GROUP: RefCell<Option<JsValue>> = RefCell::new(None);
    static RHAI_LAYER_GROUP: RefCell<Option<JsValue>> = RefCell::new(None);
    static USER_MARKER: RefCell<Option<JsValue>> = RefCell::new(None);
    static TEMP_MARKER: RefCell<Option<JsValue>> = RefCell::new(None);
    static SENSOR_DATA: RefCell<(f64, f64, f64, f64, f64)> = RefCell::new((0.0, 0.0, 0.0, 0.0, 0.0));
    static SAVED_PINS: RefCell<Vec<Pin>> = RefCell::new(Vec::new());
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    type LMap; type LLayer;
    
    #[wasm_bindgen(js_namespace = L)]
    fn map(id: &str, options: &JsValue) -> LMap;
    
    #[wasm_bindgen(method, js_name = setView)]
    fn set_view(this: &LMap, center: &js_sys::Array, zoom: f64);
    
    #[wasm_bindgen(method, js_name = flyTo)]
    fn fly_to(this: &LMap, center: &js_sys::Array, zoom: f64);
    
    #[wasm_bindgen(method, js_name = invalidateSize)]
    fn invalidate_size(this: &LMap);
    
    #[wasm_bindgen(method, js_name = on)]
    fn on_event(this: &LMap, event: &str, callback: &Closure<dyn FnMut(JsValue)>);
    
    #[wasm_bindgen(js_namespace = L)]
    fn tileLayer(url: &str, options: &JsValue) -> LLayer;
    
    #[wasm_bindgen(js_namespace = L)]
    fn layerGroup() -> LLayer;
    
    #[wasm_bindgen(js_namespace = L)]
    fn marker(latlng: &js_sys::Array, options: &JsValue) -> LLayer;
    
    #[wasm_bindgen(js_namespace = L)]
    fn circle(latlng: &js_sys::Array, options: &JsValue) -> LLayer;
    
    #[wasm_bindgen(method, js_name = addTo)]
    fn add_to(this: &LLayer, map: &JsValue) -> LLayer;
    
    #[wasm_bindgen(method, js_name = setLatLng)]
    fn set_lat_lng(this: &LLayer, latlng: &js_sys::Array);
    
    #[wasm_bindgen(method, js_name = clearLayers)]
    fn clear_layers(this: &LLayer);
    
    #[wasm_bindgen(method, js_name = remove)]
    fn remove(this: &LLayer);
    
    #[wasm_bindgen(method, js_name = bindPopup)]
    fn bind_popup(this: &LLayer, content: &str) -> LLayer;
    
    #[wasm_bindgen(method, js_name = openPopup)]
    fn open_popup(this: &LLayer) -> LLayer;
}

fn term_print(msg: &str) {
    if let Some(win) = web_sys::window() {
        if let Some(doc) = win.document() {
            if let Some(el) = doc.get_element_by_id("terminal-output") {
                let html = el.inner_html(); el.set_inner_html(&format!("{}<br>> {}", html, msg));
                let el_dyn = el.unchecked_into::<HtmlElement>(); el_dyn.set_scroll_top(el_dyn.scroll_height());
            }
        }
    }
}

fn get_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn save_pins_to_storage() {
    SAVED_PINS.with(|p| {
        if let Ok(json) = serde_json::to_string(&*p.borrow()) {
            if let Some(storage) = get_storage() {
                let _ = storage.set_item("rust_maps_pins", &json);
            }
        }
    });
}

fn load_pins_from_storage() {
    if let Some(storage) = get_storage() {
        if let Ok(Some(data)) = storage.get_item("rust_maps_pins") {
            if let Ok(pins) = serde_json::from_str::<Vec<Pin>>(&data) {
                SAVED_PINS.with(|p| *p.borrow_mut() = pins);
            }
        }
    }
}

#[wasm_bindgen]
pub fn render_pins() {
    PIN_LAYER_GROUP.with(|lg| {
        if let Some(group_val) = &*lg.borrow() {
            let group = group_val.unchecked_ref::<LLayer>();
            group.clear_layers();
            
            SAVED_PINS.with(|p| {
                for pin in p.borrow().iter() {
                    let arr = js_sys::Array::of2(&pin.lat.into(), &pin.lon.into());
                    
                    let html = format!(
                        r#"<div style="text-align:center;">
                            <b>{}</b><br><br>
                            <button onclick="window.delete_pin('{}')" style="background:#ff4444; color:white; border:none; padding:5px 10px; border-radius:4px; cursor:pointer;">Delete Pin</button>
                        </div>"#,
                        pin.title, pin.id
                    );
                    
                    marker(&arr, &js_sys::Object::new())
                        .bind_popup(&html)
                        .add_to(group_val);
                }
            });
        }
    });
}

#[wasm_bindgen]
pub fn add_pin_from_js(lat: f64, lon: f64, title: &str) {
    let id = format!("{}", web_sys::window().unwrap().performance().unwrap().now());
    let pin = Pin { id, lat, lon, title: title.to_string() };
    SAVED_PINS.with(|p| p.borrow_mut().push(pin));
    
    TEMP_MARKER.with(|tm| {
        if let Some(m) = &*tm.borrow() {
            m.unchecked_ref::<LLayer>().remove();
        }
    });
    
    save_pins_to_storage();
    render_pins();
    term_print(&format!("<span style='color: #0f0;'>Added Pin: {}</span>", title));
}

#[wasm_bindgen]
pub fn delete_pin_from_js(id: &str) {
    SAVED_PINS.with(|p| p.borrow_mut().retain(|pin| pin.id != id));
    save_pins_to_storage();
    render_pins();
    term_print("Pin deleted.");
}

#[wasm_bindgen]
pub fn execute_rhai(script: &str) {
    term_print("<i>Running script...</i>");
    let mut engine = Engine::new();
    engine.on_print(|x| term_print(x));
    
    engine.register_fn("get_lat", || -> f64 { SENSOR_DATA.with(|d| d.borrow().0) });
    engine.register_fn("get_lon", || -> f64 { SENSOR_DATA.with(|d| d.borrow().1) });
    
    engine.register_fn("get_accel", || -> Dynamic { 
        let (x,y,z) = SENSOR_DATA.with(|d| { let b = d.borrow(); (b.2, b.3, b.4) });
        let mut map = rhai::Map::new();
        map.insert("x".into(), x.into()); map.insert("y".into(), y.into()); map.insert("z".into(), z.into());
        Dynamic::from_map(map)
    });
    
    engine.register_fn("clear_drawings", || {
        RHAI_LAYER_GROUP.with(|lg| { if let Some(group) = &*lg.borrow() { group.unchecked_ref::<LLayer>().clear_layers(); }});
        term_print("Drawings cleared.");
    });
    
    engine.register_fn("save_pin", |lat: f64, lon: f64, title: &str| {
        add_pin_from_js(lat, lon, title);
    });
    
    engine.register_fn("clear_all_pins", || {
        SAVED_PINS.with(|p| p.borrow_mut().clear());
        save_pins_to_storage();
        render_pins();
        term_print("All persistent pins erased.");
    });
    
    engine.register_fn("add_circle", |lat: f64, lon: f64, radius: f64, color: &str| {
        RHAI_LAYER_GROUP.with(|lg| { if let Some(group) = &*lg.borrow() {
            let arr = js_sys::Array::of2(&lat.into(), &lon.into()); let opts = js_sys::Object::new();
            js_sys::Reflect::set(&opts, &"radius".into(), &radius.into()).unwrap();
            js_sys::Reflect::set(&opts, &"color".into(), &color.into()).unwrap();
            circle(&arr, &opts).add_to(group);
        }});
    });

    match engine.eval::<()>(script) {
        Ok(_) => term_print("<span style='color: #0f0;'>✓ Complete.</span>"),
        Err(e) => term_print(&format!("<span style='color: #f55;'>Error: {}</span>", e)),
    }
}

#[wasm_bindgen]
pub fn fly_to_user() {
    SENSOR_DATA.with(|d| {
        let (lat, lon, _, _, _) = *d.borrow();
        if lat != 0.0 && lon != 0.0 {
            MAP_INSTANCE.with(|m| {
                if let Some(map_val) = &*m.borrow() {
                    map_val.unchecked_ref::<LMap>().fly_to(&js_sys::Array::of2(&lat.into(), &lon.into()), 16.0);
                }
            });
        } else {
            term_print("<span style='color: yellow;'>GPS not locked yet.</span>");
        }
    });
}

#[wasm_bindgen]
pub fn resize_map() { MAP_INSTANCE.with(|m| { if let Some(map_val) = &*m.borrow() { map_val.unchecked_ref::<LMap>().invalidate_size(); }}); }

#[wasm_bindgen]
pub fn start_app() {
    console_error_panic_hook::set_once();
    let win = web_sys::window().expect("no window");
    
    let map_opts = js_sys::Object::new();
    js_sys::Reflect::set(&map_opts, &"zoomControl".into(), &false.into()).unwrap();

    let map_obj = map("map", &map_opts); 
    map_obj.set_view(&js_sys::Array::of2(&20.0.into(), &0.0.into()), 2.0);
    
    let opt = js_sys::Object::new(); js_sys::Reflect::set(&opt, &"attribution".into(), &"© OSM".into()).unwrap();
    tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", &opt).add_to(&map_obj.unchecked_ref::<JsValue>());

    let pin_group = layerGroup(); pin_group.add_to(&map_obj.unchecked_ref::<JsValue>());
    let rhai_group = layerGroup(); rhai_group.add_to(&map_obj.unchecked_ref::<JsValue>());
    
    PIN_LAYER_GROUP.with(|lg| *lg.borrow_mut() = Some(pin_group.unchecked_ref::<JsValue>().clone()));
    RHAI_LAYER_GROUP.with(|lg| *lg.borrow_mut() = Some(rhai_group.unchecked_ref::<JsValue>().clone()));
    MAP_INSTANCE.with(|m| *m.borrow_mut() = Some(map_obj.unchecked_ref::<JsValue>().clone()));

    let on_map_click = Closure::wrap(Box::new(move |e: JsValue| {
        if let Ok(latlng) = js_sys::Reflect::get(&e, &"latlng".into()) {
            let lat = js_sys::Reflect::get(&latlng, &"lat".into()).unwrap().as_f64().unwrap();
            let lng = js_sys::Reflect::get(&latlng, &"lng".into()).unwrap().as_f64().unwrap();
            
            TEMP_MARKER.with(|tm| {
                if let Some(m) = &*tm.borrow() { m.unchecked_ref::<LLayer>().remove(); }
            });
            
            let html = format!(
                r#"<div style="text-align:center;">
                    <b style="font-size:14px;">Save Location</b><br>
                    <input type="text" id="new-pin-title" placeholder="Pin Name" style="width:100%; margin:8px 0; padding:5px; border:1px solid #ccc; border-radius:4px;"><br>
                    <button onclick="window.submit_pin({}, {})" style="background:#007aff; color:white; border:none; padding:8px 15px; border-radius:4px; font-weight:bold; cursor:pointer; width:100%;">Save</button>
                </div>"#,
                lat, lng
            );
            
            MAP_INSTANCE.with(|m| {
                if let Some(map_val) = &*m.borrow() {
                    let new_m = marker(&js_sys::Array::of2(&lat.into(), &lng.into()), &js_sys::Object::new());
                    new_m.bind_popup(&html).add_to(map_val).open_popup();
                    TEMP_MARKER.with(|tm| *tm.borrow_mut() = Some(new_m.unchecked_ref::<JsValue>().clone()));
                }
            });
        }
    }) as Box<dyn FnMut(JsValue)>);
    map_obj.on_event("click", &on_map_click);
    on_map_click.forget();

    load_pins_from_storage();
    render_pins();

    let nav = win.navigator(); 
    if let Ok(geo) = nav.geolocation() {
        let map_val = map_obj.unchecked_ref::<JsValue>().clone();
        
        let on_gps = Closure::wrap(Box::new(move |pos: Position| {
            let lat = pos.coords().latitude(); let lon = pos.coords().longitude();
            SENSOR_DATA.with(|d| { let mut data = d.borrow_mut(); data.0 = lat; data.1 = lon; });
            
            let doc = web_sys::window().unwrap().document().unwrap();
            if let Some(el) = doc.get_element_by_id("nav-status") { 
                el.set_inner_html("GPS Locked"); 
                let _ = el.set_attribute("style", "color: #4ade80;"); 
            }
            
            let arr = js_sys::Array::of2(&lat.into(), &lon.into());
            USER_MARKER.with(|um| {
                let mut marker_opt = um.borrow_mut();
                if let Some(m) = &*marker_opt {
                    m.unchecked_ref::<LLayer>().set_lat_lng(&arr);
                } else {
                    let opts = js_sys::Object::new();
                    js_sys::Reflect::set(&opts, &"color".into(), &"white".into()).unwrap();
                    js_sys::Reflect::set(&opts, &"fillColor".into(), &"#007aff".into()).unwrap();
                    js_sys::Reflect::set(&opts, &"fillOpacity".into(), &1.0.into()).unwrap();
                    js_sys::Reflect::set(&opts, &"radius".into(), &8.0.into()).unwrap();
                    let new_marker = circle(&arr, &opts);
                    new_marker.add_to(&map_val);
                    *marker_opt = Some(new_marker.unchecked_ref::<JsValue>().clone());
                }
            });
        }) as Box<dyn FnMut(Position)>);
        let _ = geo.watch_position(on_gps.as_ref().unchecked_ref()); on_gps.forget();
    }

    let on_motion = Closure::wrap(Box::new(move |event: DeviceMotionEvent| {
        if let Some(acc) = event.acceleration() {
            let (x,y,z) = (acc.x().unwrap_or(0.0), acc.y().unwrap_or(0.0), acc.z().unwrap_or(0.0));
            SENSOR_DATA.with(|d| { let mut data = d.borrow_mut(); data.2 = x; data.3 = y; data.4 = z; });
        }
    }) as Box<dyn FnMut(DeviceMotionEvent)>);
    win.add_event_listener_with_callback("devicemotion", on_motion.as_ref().unchecked_ref()).unwrap(); 
    on_motion.forget();
}
