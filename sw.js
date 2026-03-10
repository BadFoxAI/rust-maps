const CACHE_NAME = 'rust-maps-v2';
const ASSETS = [
  './', './index.html', './main.js', './manifest.json',
  './pkg/rust_maps.js', './pkg/rust_maps_bg.wasm',
  './lib/leaflet.css', './lib/leaflet.js',
  './lib/images/marker-icon.png', './lib/images/marker-shadow.png'
];

self.addEventListener('install', e => {
  e.waitUntil(caches.open(CACHE_NAME).then(c => c.addAll(ASSETS)));
});
self.addEventListener('fetch', e => {
  e.respondWith(caches.match(e.request).then(r => r || fetch(e.request)));
});
