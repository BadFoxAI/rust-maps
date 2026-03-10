/* tslint:disable */
/* eslint-disable */

export function add_pin_from_js(lat: number, lon: number, title: string): void;

export function delete_pin_from_js(id: string): void;

export function execute_rhai(script: string): void;

export function fly_to_user(): void;

export function render_pins(): void;

export function resize_map(): void;

export function start_app(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly add_pin_from_js: (a: number, b: number, c: number, d: number) => void;
    readonly delete_pin_from_js: (a: number, b: number) => void;
    readonly execute_rhai: (a: number, b: number) => void;
    readonly fly_to_user: () => void;
    readonly render_pins: () => void;
    readonly resize_map: () => void;
    readonly start_app: () => void;
    readonly wasm_bindgen__closure__destroy__h19b7dfd947f9adba: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h31ce8e91e126c701: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h31ce8e91e126c701_1: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h31ce8e91e126c701_2: (a: number, b: number, c: any) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
