let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

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

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_0.set(idx, obj);
    return idx;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getDataViewMemory0();
    for (let i = 0; i < array.length; i++) {
        mem.setUint32(ptr + 4 * i, addToExternrefTable0(array[i]), true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_0.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
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
     * @param {Rgb} primary
     * @param {Rgb} secondary
     * @param {Rgb} tertiary
     * @param {Rgb} outline
     * @param {Rgb} on_primary_container
     * @param {Rgb} on_secondary_container
     * @param {Rgb} on_tertiary_container
     * @param {Rgb} on_background
     */
    constructor(primary, secondary, tertiary, outline, on_primary_container, on_secondary_container, on_tertiary_container, on_background) {
        _assertClass(primary, Rgb);
        var ptr0 = primary.__destroy_into_raw();
        _assertClass(secondary, Rgb);
        var ptr1 = secondary.__destroy_into_raw();
        _assertClass(tertiary, Rgb);
        var ptr2 = tertiary.__destroy_into_raw();
        _assertClass(outline, Rgb);
        var ptr3 = outline.__destroy_into_raw();
        _assertClass(on_primary_container, Rgb);
        var ptr4 = on_primary_container.__destroy_into_raw();
        _assertClass(on_secondary_container, Rgb);
        var ptr5 = on_secondary_container.__destroy_into_raw();
        _assertClass(on_tertiary_container, Rgb);
        var ptr6 = on_tertiary_container.__destroy_into_raw();
        _assertClass(on_background, Rgb);
        var ptr7 = on_background.__destroy_into_raw();
        const ret = wasm.themecolors_new(ptr0, ptr1, ptr2, ptr3, ptr4, ptr5, ptr6, ptr7);
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
    /**
     * @returns {number}
     */
    get pt() {
        const ret = wasm.__wbg_get_typststate_pt(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set pt(arg0) {
        wasm.__wbg_set_typststate_pt(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get size() {
        const ret = wasm.__wbg_get_typststate_size(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set size(arg0) {
        wasm.__wbg_set_typststate_size(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {ThemeColors}
     */
    get theme() {
        const ret = wasm.__wbg_get_typststate_theme(this.__wbg_ptr);
        return ThemeColors.__wrap(ret);
    }
    /**
     * @param {ThemeColors} arg0
     */
    set theme(arg0) {
        _assertClass(arg0, ThemeColors);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_typststate_theme(this.__wbg_ptr, ptr0);
    }
    constructor() {
        const ret = wasm.typststate_new();
        this.__wbg_ptr = ret >>> 0;
        TypstStateFinalization.register(this, this.__wbg_ptr, this);
        return this;
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
     * @param {(PackageFile)[]} files
     */
    installPackage(spec, files) {
        const ptr0 = passStringToWasm0(spec, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayJsValueToWasm0(files, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.typststate_installPackage(this.__wbg_ptr, ptr0, len0, ptr1, len1);
    }
    /**
     * @param {FileId} id
     * @param {string} text
     * @returns {SyncResult}
     */
    sync(id, text) {
        _assertClass(id, FileId);
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.typststate_sync(this.__wbg_ptr, id.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {FileId} id
     * @returns {string}
     */
    renderPdf(id) {
        let deferred1_0;
        let deferred1_1;
        try {
            _assertClass(id, FileId);
            const ret = wasm.typststate_renderPdf(this.__wbg_ptr, id.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} index
     * @param {number} x
     * @param {number} y
     * @returns {TypstJump | undefined}
     */
    click(index, x, y) {
        const ret = wasm.typststate_click(this.__wbg_ptr, index, x, y);
        return ret;
    }
    /**
     * @param {number} cursor
     * @param {boolean} explicit
     * @returns {any}
     */
    autocomplete(cursor, explicit) {
        const ret = wasm.typststate_autocomplete(this.__wbg_ptr, cursor, explicit);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * @param {number | undefined} [width]
     * @param {number | undefined} [height]
     */
    resize(width, height) {
        wasm.typststate_resize(this.__wbg_ptr, !isLikeNone(width), isLikeNone(width) ? 0 : width, !isLikeNone(height), isLikeNone(height) ? 0 : height);
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
    imports.wbg.__wbg_packagefile_unwrap = function(arg0) {
        const ret = PackageFile.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_error_eda0b57859301c0b = function(arg0, arg1) {
        console.error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_log_f7a82a92110d52d2 = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
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
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return ret;
    };
    imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
        const ret = BigInt.asUintN(64, arg0);
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbg_set_841ac57cff3d672b = function(arg0, arg1, arg2) {
        arg0[arg1] = arg2;
    };
    imports.wbg.__wbg_set_f975102236d3c502 = function(arg0, arg1, arg2) {
        arg0[arg1] = arg2;
    };
    imports.wbg.__wbg_new_034f913e7636e987 = function() {
        const ret = new Array();
        return ret;
    };
    imports.wbg.__wbg_new_e69b5f66fda8f13c = function() {
        const ret = new Object();
        return ret;
    };
    imports.wbg.__wbg_set_425e70f7c64ac962 = function(arg0, arg1, arg2) {
        arg0[arg1 >>> 0] = arg2;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_0;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
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
