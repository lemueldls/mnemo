let wasm;

let WASM_VECTOR_LEN = 0;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_3.set(idx, obj);
    return idx;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_3.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

export function start() {
    wasm.start();
}

const FileIdFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_fileid_free(ptr >>> 0, 1));

export class FileId {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(FileId.prototype);
        obj.__wbg_ptr = ptr;
        FileIdFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        FileIdFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_fileid_free(ptr, 0);
    }
}

const PackageFileFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_packagefile_free(ptr >>> 0, 1));

export class PackageFile {

    static __unwrap(jsValue) {
        if (!(jsValue instanceof PackageFile)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PackageFileFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_packagefile_free(ptr, 0);
    }
    /**
     * @param {string} path
     * @param {Uint8Array} content
     */
    constructor(path, content) {
        const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(content, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.packagefile_new(ptr0, len0, ptr1, len1);
        this.__wbg_ptr = ret >>> 0;
        PackageFileFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const RgbFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rgb_free(ptr >>> 0, 1));

export class Rgb {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RgbFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rgb_free(ptr, 0);
    }
    /**
     * @param {number} r
     * @param {number} g
     * @param {number} b
     */
    constructor(r, g, b) {
        const ret = wasm.rgb_new(r, g, b);
        this.__wbg_ptr = ret >>> 0;
        RgbFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {string}
     */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.rgb_toString(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const ThemeColorsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_themecolors_free(ptr >>> 0, 1));

export class ThemeColors {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ThemeColors.prototype);
        obj.__wbg_ptr = ptr;
        ThemeColorsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ThemeColorsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_themecolors_free(ptr, 0);
    }
    /**
     * @param {Rgb} background
     * @param {Rgb} on_background
     * @param {Rgb} outline
     * @param {Rgb} outline_variant
     * @param {Rgb} primary
     * @param {Rgb} on_primary
     * @param {Rgb} primary_container
     * @param {Rgb} on_primary_container
     * @param {Rgb} secondary
     * @param {Rgb} on_secondary
     * @param {Rgb} secondary_container
     * @param {Rgb} on_secondary_container
     * @param {Rgb} tertiary
     * @param {Rgb} on_tertiary
     * @param {Rgb} tertiary_container
     * @param {Rgb} on_tertiary_container
     * @param {Rgb} error
     * @param {Rgb} on_error
     * @param {Rgb} error_container
     * @param {Rgb} on_error_container
     */
    constructor(background, on_background, outline, outline_variant, primary, on_primary, primary_container, on_primary_container, secondary, on_secondary, secondary_container, on_secondary_container, tertiary, on_tertiary, tertiary_container, on_tertiary_container, error, on_error, error_container, on_error_container) {
        _assertClass(background, Rgb);
        var ptr0 = background.__destroy_into_raw();
        _assertClass(on_background, Rgb);
        var ptr1 = on_background.__destroy_into_raw();
        _assertClass(outline, Rgb);
        var ptr2 = outline.__destroy_into_raw();
        _assertClass(outline_variant, Rgb);
        var ptr3 = outline_variant.__destroy_into_raw();
        _assertClass(primary, Rgb);
        var ptr4 = primary.__destroy_into_raw();
        _assertClass(on_primary, Rgb);
        var ptr5 = on_primary.__destroy_into_raw();
        _assertClass(primary_container, Rgb);
        var ptr6 = primary_container.__destroy_into_raw();
        _assertClass(on_primary_container, Rgb);
        var ptr7 = on_primary_container.__destroy_into_raw();
        _assertClass(secondary, Rgb);
        var ptr8 = secondary.__destroy_into_raw();
        _assertClass(on_secondary, Rgb);
        var ptr9 = on_secondary.__destroy_into_raw();
        _assertClass(secondary_container, Rgb);
        var ptr10 = secondary_container.__destroy_into_raw();
        _assertClass(on_secondary_container, Rgb);
        var ptr11 = on_secondary_container.__destroy_into_raw();
        _assertClass(tertiary, Rgb);
        var ptr12 = tertiary.__destroy_into_raw();
        _assertClass(on_tertiary, Rgb);
        var ptr13 = on_tertiary.__destroy_into_raw();
        _assertClass(tertiary_container, Rgb);
        var ptr14 = tertiary_container.__destroy_into_raw();
        _assertClass(on_tertiary_container, Rgb);
        var ptr15 = on_tertiary_container.__destroy_into_raw();
        _assertClass(error, Rgb);
        var ptr16 = error.__destroy_into_raw();
        _assertClass(on_error, Rgb);
        var ptr17 = on_error.__destroy_into_raw();
        _assertClass(error_container, Rgb);
        var ptr18 = error_container.__destroy_into_raw();
        _assertClass(on_error_container, Rgb);
        var ptr19 = on_error_container.__destroy_into_raw();
        const ret = wasm.themecolors_new(ptr0, ptr1, ptr2, ptr3, ptr4, ptr5, ptr6, ptr7, ptr8, ptr9, ptr10, ptr11, ptr12, ptr13, ptr14, ptr15, ptr16, ptr17, ptr18, ptr19);
        this.__wbg_ptr = ret >>> 0;
        ThemeColorsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const TypstStateFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_typststate_free(ptr >>> 0, 1));

export class TypstState {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TypstStateFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_typststate_free(ptr, 0);
    }
    constructor() {
        const ret = wasm.typststate_new();
        this.__wbg_ptr = ret >>> 0;
        TypstStateFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    pt() {
        const ret = wasm.typststate_pt(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} pt
     */
    setPt(pt) {
        wasm.typststate_setPt(this.__wbg_ptr, pt);
    }
    /**
     * @returns {number}
     */
    size() {
        const ret = wasm.typststate_size(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} size
     */
    setSize(size) {
        wasm.typststate_setSize(this.__wbg_ptr, size);
    }
    /**
     * @returns {ThemeColors}
     */
    theme() {
        const ret = wasm.typststate_theme(this.__wbg_ptr);
        return ThemeColors.__wrap(ret);
    }
    /**
     * @param {ThemeColors} theme
     */
    setTheme(theme) {
        _assertClass(theme, ThemeColors);
        var ptr0 = theme.__destroy_into_raw();
        wasm.typststate_setTheme(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {string} path
     * @param {string} text
     * @returns {FileId}
     */
    insertFile(path, text) {
        const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.typststate_insertFile(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        return FileId.__wrap(ret);
    }
    /**
     * @param {string} spec
     * @param {PackageFile[]} files
     */
    installPackage(spec, files) {
        const ptr0 = passStringToWasm0(spec, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayJsValueToWasm0(files, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.typststate_installPackage(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {Uint8Array} bytes
     */
    installFont(bytes) {
        const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.typststate_installFont(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {FileId} id
     * @param {string} text
     * @param {string} prelude
     * @returns {CompileResult}
     */
    compile(id, text, prelude) {
        _assertClass(id, FileId);
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(prelude, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.typststate_compile(this.__wbg_ptr, id.__wbg_ptr, ptr0, len0, ptr1, len1);
        return ret;
    }
    /**
     * @param {number} x
     * @param {number} y
     * @returns {TypstJump | undefined}
     */
    click(x, y) {
        const ret = wasm.typststate_click(this.__wbg_ptr, x, y);
        return ret;
    }
    /**
     * @param {number} aux_cursor_utf16
     * @param {boolean} explicit
     * @returns {Autocomplete | undefined}
     */
    autocomplete(aux_cursor_utf16, explicit) {
        const ret = wasm.typststate_autocomplete(this.__wbg_ptr, aux_cursor_utf16, explicit);
        return ret;
    }
    /**
     * @param {number | null} [width]
     * @param {number | null} [height]
     */
    resize(width, height) {
        wasm.typststate_resize(this.__wbg_ptr, !isLikeNone(width), isLikeNone(width) ? 0 : width, !isLikeNone(height), isLikeNone(height) ? 0 : height);
    }
    /**
     * @param {FileId} id
     * @returns {RenderPdfResult}
     */
    renderPdf(id) {
        _assertClass(id, FileId);
        const ret = wasm.typststate_renderPdf(this.__wbg_ptr, id.__wbg_ptr);
        return ret;
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_String_8f0eb39a4a4c2f66 = function(arg0, arg1) {
        const ret = String(arg1);
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_error_7e97ac6aa2a9e682 = function(arg0, arg1) {
        console.error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_new_405e22f390576ce2 = function() {
        const ret = new Object();
        return ret;
    };
    imports.wbg.__wbg_new_78feb108b6472713 = function() {
        const ret = new Array();
        return ret;
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_packagefile_unwrap = function(arg0) {
        const ret = PackageFile.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_set_37837023f3d740e8 = function(arg0, arg1, arg2) {
        arg0[arg1 >>> 0] = arg2;
    };
    imports.wbg.__wbg_set_3f1d0b984ed272ed = function(arg0, arg1, arg2) {
        arg0[arg1] = arg2;
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
        const ret = BigInt.asUintN(64, arg0);
        return ret;
    };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_3;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('mnemo_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
