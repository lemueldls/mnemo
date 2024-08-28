let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
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
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

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
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let cachedUint32Memory0 = null;

function getUint32Memory0() {
    if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32Memory0;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getUint32Memory0();
    for (let i = 0; i < array.length; i++) {
        mem[ptr / 4 + i] = addHeapObject(array[i]);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
*/
export function start() {
    wasm.start();
}

const PackageFileFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_packagefile_free(ptr >>> 0));
/**
*/
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
        wasm.__wbg_packagefile_free(ptr);
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
        return this;
    }
}

const RgbFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rgb_free(ptr >>> 0));
/**
*/
export class Rgb {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Rgb.prototype);
        obj.__wbg_ptr = ptr;
        RgbFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RgbFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rgb_free(ptr);
    }
    /**
    * @param {number} r
    * @param {number} g
    * @param {number} b
    */
    constructor(r, g, b) {
        const ret = wasm.rgb_new(r, g, b);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
}

const TypstStateFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_typststate_free(ptr >>> 0));
/**
*/
export class TypstState {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TypstStateFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_typststate_free(ptr);
    }
    /**
    * @returns {Rgb}
    */
    get color() {
        const ret = wasm.__wbg_get_typststate_color(this.__wbg_ptr);
        return Rgb.__wrap(ret);
    }
    /**
    * @param {Rgb} arg0
    */
    set color(arg0) {
        _assertClass(arg0, Rgb);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_typststate_color(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {Rgb}
    */
    get stroke() {
        const ret = wasm.__wbg_get_typststate_stroke(this.__wbg_ptr);
        return Rgb.__wrap(ret);
    }
    /**
    * @param {Rgb} arg0
    */
    set stroke(arg0) {
        _assertClass(arg0, Rgb);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_typststate_stroke(this.__wbg_ptr, ptr0);
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
    * @returns {Rgb}
    */
    get h1() {
        const ret = wasm.__wbg_get_typststate_h1(this.__wbg_ptr);
        return Rgb.__wrap(ret);
    }
    /**
    * @param {Rgb} arg0
    */
    set h1(arg0) {
        _assertClass(arg0, Rgb);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_typststate_h1(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {Rgb}
    */
    get h2() {
        const ret = wasm.__wbg_get_typststate_h2(this.__wbg_ptr);
        return Rgb.__wrap(ret);
    }
    /**
    * @param {Rgb} arg0
    */
    set h2(arg0) {
        _assertClass(arg0, Rgb);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_typststate_h2(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {Rgb}
    */
    get h3() {
        const ret = wasm.__wbg_get_typststate_h3(this.__wbg_ptr);
        return Rgb.__wrap(ret);
    }
    /**
    * @param {Rgb} arg0
    */
    set h3(arg0) {
        _assertClass(arg0, Rgb);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_typststate_h3(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {Rgb}
    */
    get h4() {
        const ret = wasm.__wbg_get_typststate_h4(this.__wbg_ptr);
        return Rgb.__wrap(ret);
    }
    /**
    * @param {Rgb} arg0
    */
    set h4(arg0) {
        _assertClass(arg0, Rgb);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_typststate_h4(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {Rgb}
    */
    get h5() {
        const ret = wasm.__wbg_get_typststate_h5(this.__wbg_ptr);
        return Rgb.__wrap(ret);
    }
    /**
    * @param {Rgb} arg0
    */
    set h5(arg0) {
        _assertClass(arg0, Rgb);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_typststate_h5(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {Rgb}
    */
    get h6() {
        const ret = wasm.__wbg_get_typststate_h6(this.__wbg_ptr);
        return Rgb.__wrap(ret);
    }
    /**
    * @param {Rgb} arg0
    */
    set h6(arg0) {
        _assertClass(arg0, Rgb);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_typststate_h6(this.__wbg_ptr, ptr0);
    }
    /**
    */
    constructor() {
        const ret = wasm.typststate_new();
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @param {string} path
    * @param {string} text
    */
    setMain(path, text) {
        const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.typststate_setMain(this.__wbg_ptr, ptr0, len0, ptr1, len1);
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
    * @param {string} text
    * @returns {SyncResult}
    */
    sync(text) {
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.typststate_sync(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * @param {number} index
    * @param {number} x
    * @param {number} y
    * @returns {TypstJump | undefined}
    */
    click(index, x, y) {
        const ret = wasm.typststate_click(this.__wbg_ptr, index, x, y);
        return takeObject(ret);
    }
    /**
    * @param {number} cursor
    * @param {boolean} explicit
    * @returns {any}
    */
    autocomplete(cursor, explicit) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.typststate_autocomplete(retptr, this.__wbg_ptr, cursor, explicit);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

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
        const ret = PackageFile.__unwrap(takeObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_error_eda0b57859301c0b = function(arg0, arg1) {
        console.error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_log_f7a82a92110d52d2 = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
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
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
        const ret = BigInt.asUintN(64, arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_841ac57cff3d672b = function(arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    };
    imports.wbg.__wbg_set_f975102236d3c502 = function(arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    };
    imports.wbg.__wbg_new_16b304a2cfa7ff4a = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_72fb9a18b5ae2624 = function() {
        const ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_d4638f722068f043 = function(arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint32Memory0 = null;
    cachedUint8Memory0 = null;

    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('mnemo_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
