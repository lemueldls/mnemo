/* tslint:disable */
/* eslint-disable */
/**
*/
export function start(): void;
export type SyncResult = { kind: "ok"; data: RangedRender[] } | { kind: "error"; data: string[] };

export interface RangedRender {
    index: number;
    block: Block;
    render: string;
}

export interface Block {
    range: { start: number; end: number };
    offset: number;
}

export type TypstJump = { type: "Source"; position: number };

export type TypstCompletionKind = "Syntax" | "Function" | "Parameter" | "Constant" | "Symbol" | "Type";

export interface TypstCompletion {
    kind: TypstCompletionKind;
    label: string;
    apply: string | undefined;
    detail: string | undefined;
}

/**
*/
export class FileId {
  free(): void;
}
/**
*/
export class PackageFile {
  free(): void;
/**
* @param {string} path
* @param {Uint8Array} content
*/
  constructor(path: string, content: Uint8Array);
}
/**
*/
export class Rgb {
  free(): void;
/**
* @param {number} r
* @param {number} g
* @param {number} b
*/
  constructor(r: number, g: number, b: number);
}
/**
*/
export class TypstState {
  free(): void;
/**
*/
  constructor();
/**
* @param {string} path
* @param {string} text
* @returns {FileId}
*/
  insertFile(path: string, text: string): FileId;
/**
* @param {string} spec
* @param {(PackageFile)[]} files
*/
  installPackage(spec: string, files: (PackageFile)[]): void;
/**
* @param {FileId} id
* @param {string} text
* @returns {SyncResult}
*/
  sync(id: FileId, text: string): SyncResult;
/**
* @param {number} index
* @param {number} x
* @param {number} y
* @returns {TypstJump | undefined}
*/
  click(index: number, x: number, y: number): TypstJump | undefined;
/**
* @param {number} cursor
* @param {boolean} explicit
* @returns {any}
*/
  autocomplete(cursor: number, explicit: boolean): any;
/**
* @param {number | undefined} [width]
* @param {number | undefined} [height]
*/
  resize(width?: number, height?: number): void;
/**
*/
  color: Rgb;
/**
*/
  h1: Rgb;
/**
*/
  h2: Rgb;
/**
*/
  h3: Rgb;
/**
*/
  h4: Rgb;
/**
*/
  h5: Rgb;
/**
*/
  h6: Rgb;
/**
*/
  pt: number;
/**
*/
  size: number;
/**
*/
  stroke: Rgb;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_packagefile_free: (a: number) => void;
  readonly packagefile_new: (a: number, b: number, c: number, d: number) => number;
  readonly rgb_new: (a: number, b: number, c: number) => number;
  readonly __wbg_typststate_free: (a: number) => void;
  readonly __wbg_get_typststate_color: (a: number) => number;
  readonly __wbg_set_typststate_color: (a: number, b: number) => void;
  readonly __wbg_get_typststate_stroke: (a: number) => number;
  readonly __wbg_set_typststate_stroke: (a: number, b: number) => void;
  readonly __wbg_get_typststate_pt: (a: number) => number;
  readonly __wbg_set_typststate_pt: (a: number, b: number) => void;
  readonly __wbg_get_typststate_size: (a: number) => number;
  readonly __wbg_set_typststate_size: (a: number, b: number) => void;
  readonly __wbg_get_typststate_h1: (a: number) => number;
  readonly __wbg_set_typststate_h1: (a: number, b: number) => void;
  readonly __wbg_get_typststate_h2: (a: number) => number;
  readonly __wbg_set_typststate_h2: (a: number, b: number) => void;
  readonly __wbg_get_typststate_h3: (a: number) => number;
  readonly __wbg_set_typststate_h3: (a: number, b: number) => void;
  readonly __wbg_get_typststate_h4: (a: number) => number;
  readonly __wbg_set_typststate_h4: (a: number, b: number) => void;
  readonly __wbg_get_typststate_h5: (a: number) => number;
  readonly __wbg_set_typststate_h5: (a: number, b: number) => void;
  readonly __wbg_get_typststate_h6: (a: number) => number;
  readonly __wbg_set_typststate_h6: (a: number, b: number) => void;
  readonly __wbg_fileid_free: (a: number) => void;
  readonly typststate_new: () => number;
  readonly typststate_insertFile: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly typststate_installPackage: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly typststate_sync: (a: number, b: number, c: number, d: number) => number;
  readonly typststate_click: (a: number, b: number, c: number, d: number) => number;
  readonly typststate_autocomplete: (a: number, b: number, c: number, d: number) => void;
  readonly typststate_resize: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbg_rgb_free: (a: number) => void;
  readonly start: () => void;
  readonly qcms_profile_is_bogus: (a: number) => number;
  readonly qcms_white_point_sRGB: (a: number) => void;
  readonly qcms_profile_precache_output_transform: (a: number) => void;
  readonly qcms_transform_data_rgb_out_lut_precache: (a: number, b: number, c: number, d: number) => void;
  readonly qcms_transform_data_rgba_out_lut_precache: (a: number, b: number, c: number, d: number) => void;
  readonly qcms_transform_data_bgra_out_lut_precache: (a: number, b: number, c: number, d: number) => void;
  readonly qcms_transform_data_rgb_out_lut: (a: number, b: number, c: number, d: number) => void;
  readonly qcms_transform_data_rgba_out_lut: (a: number, b: number, c: number, d: number) => void;
  readonly qcms_transform_data_bgra_out_lut: (a: number, b: number, c: number, d: number) => void;
  readonly qcms_transform_release: (a: number) => void;
  readonly qcms_enable_iccv4: () => void;
  readonly lut_interp_linear16: (a: number, b: number, c: number) => number;
  readonly lut_inverse_interp16: (a: number, b: number, c: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
