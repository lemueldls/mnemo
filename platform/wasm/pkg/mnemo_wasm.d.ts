/* tslint:disable */
/* eslint-disable */
export function start(): void;
export interface TypstDiagnostic {
    range: { start: number; end: number };
    severity: TypstDiagnosticSeverity;
    message: string;
    hints: string[];
}

export type TypstDiagnosticSeverity = "error" | "warning" | "info" | "hint";

export type TypstJump = { type: "File"; position: number };

export type TypstCompletionKind = "syntax" | "func" | "type" | "param" | "constant" | "path" | "package" | "label" | "font" | "symbol";

export interface TypstCompletion {
    type: TypstCompletionKind;
    label: string;
    apply: string | undefined;
    detail: string | undefined;
}

export interface CompileResult {
    frames: RangedFrame[];
    diagnostics: TypstDiagnostic[];
}

export interface RenderPdfResult {
    bytes: number[] | undefined;
    diagnostics: TypstDiagnostic[];
}

export type TypstError = EcoString;

export interface Autocomplete {
    offset: number;
    completions: TypstCompletion[];
}

export interface RangedFrame {
    range: { start: number; end: number };
    render: FrameRender;
}

export interface FrameRender {
    encoding: string;
    height: number;
    offsetHeight: number;
}

export class FileId {
  private constructor();
  free(): void;
}
export class PackageFile {
  free(): void;
  constructor(path: string, content: Uint8Array);
}
export class Rgb {
  free(): void;
  constructor(r: number, g: number, b: number);
  toString(): string;
}
export class ThemeColors {
  free(): void;
  constructor(primary: Rgb, secondary: Rgb, tertiary: Rgb, outline: Rgb, on_primary_container: Rgb, on_secondary_container: Rgb, on_tertiary_container: Rgb, on_background: Rgb);
}
export class TypstState {
  free(): void;
  constructor();
  pt(): number;
  setPt(pt: number): void;
  size(): number;
  setSize(size: number): void;
  theme(): ThemeColors;
  setTheme(theme: ThemeColors): void;
  insertFile(path: string, text: string): FileId;
  installPackage(spec: string, files: PackageFile[]): void;
  installFont(bytes: Uint8Array): void;
  compile(id: FileId, text: string, prelude: string): CompileResult;
  renderPdf(id: FileId): RenderPdfResult;
  click(x: number, y: number): TypstJump | undefined;
  autocomplete(aux_cursor_utf16: number, explicit: boolean): Autocomplete;
  resize(width?: number | null, height?: number | null): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_packagefile_free: (a: number, b: number) => void;
  readonly packagefile_new: (a: number, b: number, c: number, d: number) => number;
  readonly __wbg_themecolors_free: (a: number, b: number) => void;
  readonly themecolors_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly rgb_new: (a: number, b: number, c: number) => number;
  readonly rgb_toString: (a: number) => [number, number];
  readonly __wbg_typststate_free: (a: number, b: number) => void;
  readonly __wbg_fileid_free: (a: number, b: number) => void;
  readonly typststate_new: () => number;
  readonly typststate_pt: (a: number) => number;
  readonly typststate_setPt: (a: number, b: number) => void;
  readonly typststate_size: (a: number) => number;
  readonly typststate_setSize: (a: number, b: number) => void;
  readonly typststate_theme: (a: number) => number;
  readonly typststate_setTheme: (a: number, b: number) => void;
  readonly typststate_insertFile: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly typststate_installPackage: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly typststate_installFont: (a: number, b: number, c: number) => void;
  readonly typststate_compile: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
  readonly typststate_renderPdf: (a: number, b: number) => any;
  readonly typststate_click: (a: number, b: number, c: number) => any;
  readonly typststate_autocomplete: (a: number, b: number, c: number) => any;
  readonly typststate_resize: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly start: () => void;
  readonly __wbg_rgb_free: (a: number, b: number) => void;
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
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly __externref_table_alloc: () => number;
  readonly __externref_table_dealloc: (a: number) => void;
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
