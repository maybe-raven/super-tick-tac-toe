let wasm_bindgen;
(function() {
    const __exports = {};
    let script_src;
    if (typeof document !== 'undefined' && document.currentScript !== null) {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }
    let wasm = undefined;

    function addToExternrefTable0(obj) {
        const idx = wasm.__externref_table_alloc();
        wasm.__wbindgen_export_2.set(idx, obj);
        return idx;
    }

    function handleError(f, args) {
        try {
            return f.apply(this, args);
        } catch (e) {
            const idx = addToExternrefTable0(e);
            wasm.__wbindgen_exn_store(idx);
        }
    }

    let cachedDataViewMemory0 = null;

    function getDataViewMemory0() {
        if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
            cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
        }
        return cachedDataViewMemory0;
    }

    function getArrayJsValueFromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        const mem = getDataViewMemory0();
        const result = [];
        for (let i = ptr; i < ptr + 4 * len; i += 4) {
            result.push(wasm.__wbindgen_export_2.get(mem.getUint32(i, true)));
        }
        wasm.__externref_drop_slice(ptr, len);
        return result;
    }

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

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(state => {
        wasm.__wbindgen_export_5.get(state.dtor)(state.a, state.b)
    });

    function makeClosure(arg0, arg1, dtor, f) {
        const state = { a: arg0, b: arg1, cnt: 1, dtor };
        const real = (...args) => {
            // First up with a closure we increment the internal reference
            // count. This ensures that the Rust closure environment won't
            // be deallocated while we're invoking it.
            state.cnt++;
            try {
                return f(state.a, state.b, ...args);
            } finally {
                if (--state.cnt === 0) {
                    wasm.__wbindgen_export_5.get(state.dtor)(state.a, state.b);
                    state.a = 0;
                    CLOSURE_DTORS.unregister(state);
                }
            }
        };
        real.original = state;
        CLOSURE_DTORS.register(real, state, state);
        return real;
    }

    function makeMutClosure(arg0, arg1, dtor, f) {
        const state = { a: arg0, b: arg1, cnt: 1, dtor };
        const real = (...args) => {
            // First up with a closure we increment the internal reference
            // count. This ensures that the Rust closure environment won't
            // be deallocated while we're invoking it.
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return f(a, state.b, ...args);
            } finally {
                if (--state.cnt === 0) {
                    wasm.__wbindgen_export_5.get(state.dtor)(a, state.b);
                    CLOSURE_DTORS.unregister(state);
                } else {
                    state.a = a;
                }
            }
        };
        real.original = state;
        CLOSURE_DTORS.register(real, state, state);
        return real;
    }
    function __wbg_adapter_28(arg0, arg1, arg2) {
        wasm.closure35_externref_shim(arg0, arg1, arg2);
    }

    function __wbg_adapter_31(arg0, arg1, arg2) {
        wasm.closure72_externref_shim(arg0, arg1, arg2);
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
        imports.wbg.__wbg_buffer_609cc3eee51ed158 = function(arg0) {
            const ret = arg0.buffer;
            return ret;
        };
        imports.wbg.__wbg_call_672a4d21634d4a24 = function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments) };
        imports.wbg.__wbg_call_7cccdd69e0791ae2 = function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments) };
        imports.wbg.__wbg_close_a1fb0ff4e390caa1 = function(arg0) {
            arg0.close();
        };
        imports.wbg.__wbg_crypto_ed58b8e10a292839 = function(arg0) {
            const ret = arg0.crypto;
            return ret;
        };
        imports.wbg.__wbg_data_432d9c3df2630942 = function(arg0) {
            const ret = arg0.data;
            return ret;
        };
        imports.wbg.__wbg_getRandomValues_bcb4912f16000dc4 = function() { return handleError(function (arg0, arg1) {
            arg0.getRandomValues(arg1);
        }, arguments) };
        imports.wbg.__wbg_length_a446193dc22c12f8 = function(arg0) {
            const ret = arg0.length;
            return ret;
        };
        imports.wbg.__wbg_log_c3d56bb0009edd6a = function(arg0, arg1) {
            var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
            wasm.__wbindgen_free(arg0, arg1 * 4, 4);
            console.log(...v0);
        };
        imports.wbg.__wbg_msCrypto_0a36e2ec3a343d26 = function(arg0) {
            const ret = arg0.msCrypto;
            return ret;
        };
        imports.wbg.__wbg_new_a12002a7f91c75be = function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        };
        imports.wbg.__wbg_newnoargs_105ed471475aaf50 = function(arg0, arg1) {
            const ret = new Function(getStringFromWasm0(arg0, arg1));
            return ret;
        };
        imports.wbg.__wbg_newwithbyteoffsetandlength_d97e637ebe145a9a = function(arg0, arg1, arg2) {
            const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
            return ret;
        };
        imports.wbg.__wbg_newwithlength_a381634e90c276d4 = function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return ret;
        };
        imports.wbg.__wbg_node_02999533c4ea02e3 = function(arg0) {
            const ret = arg0.node;
            return ret;
        };
        imports.wbg.__wbg_now_2c95c9de01293173 = function(arg0) {
            const ret = arg0.now();
            return ret;
        };
        imports.wbg.__wbg_performance_7a3ffd0b17f663ad = function(arg0) {
            const ret = arg0.performance;
            return ret;
        };
        imports.wbg.__wbg_postMessage_83a8d58d3fcb6c13 = function() { return handleError(function (arg0, arg1) {
            arg0.postMessage(arg1);
        }, arguments) };
        imports.wbg.__wbg_process_5c1d670bc53614b8 = function(arg0) {
            const ret = arg0.process;
            return ret;
        };
        imports.wbg.__wbg_queueMicrotask_97d92b4fcc8a61c5 = function(arg0) {
            queueMicrotask(arg0);
        };
        imports.wbg.__wbg_queueMicrotask_d3219def82552485 = function(arg0) {
            const ret = arg0.queueMicrotask;
            return ret;
        };
        imports.wbg.__wbg_randomFillSync_ab2cfe79ebbf2740 = function() { return handleError(function (arg0, arg1) {
            arg0.randomFillSync(arg1);
        }, arguments) };
        imports.wbg.__wbg_require_79b1e9274cde3c87 = function() { return handleError(function () {
            const ret = module.require;
            return ret;
        }, arguments) };
        imports.wbg.__wbg_resolve_4851785c9c5f573d = function(arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        };
        imports.wbg.__wbg_set_65595bdd868b3009 = function(arg0, arg1, arg2) {
            arg0.set(arg1, arg2 >>> 0);
        };
        imports.wbg.__wbg_setonmessage_7530ae0596a01ccb = function(arg0, arg1) {
            arg0.onmessage = arg1;
        };
        imports.wbg.__wbg_static_accessor_GLOBAL_88a902d13a557d07 = function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        };
        imports.wbg.__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0 = function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        };
        imports.wbg.__wbg_static_accessor_SELF_37c5d418e4bf5819 = function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        };
        imports.wbg.__wbg_static_accessor_WINDOW_5de37043a91a9c40 = function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        };
        imports.wbg.__wbg_subarray_aa9065fa9dc5df96 = function(arg0, arg1, arg2) {
            const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
            return ret;
        };
        imports.wbg.__wbg_then_44b73946d2fb3e7d = function(arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        };
        imports.wbg.__wbg_versions_c71aa1626a93e0a1 = function(arg0) {
            const ret = arg0.versions;
            return ret;
        };
        imports.wbg.__wbindgen_array_new = function() {
            const ret = [];
            return ret;
        };
        imports.wbg.__wbindgen_array_push = function(arg0, arg1) {
            arg0.push(arg1);
        };
        imports.wbg.__wbindgen_cb_drop = function(arg0) {
            const obj = arg0.original;
            if (obj.cnt-- == 1) {
                obj.a = 0;
                return true;
            }
            const ret = false;
            return ret;
        };
        imports.wbg.__wbindgen_closure_wrapper109 = function(arg0, arg1, arg2) {
            const ret = makeClosure(arg0, arg1, 36, __wbg_adapter_28);
            return ret;
        };
        imports.wbg.__wbindgen_closure_wrapper194 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 73, __wbg_adapter_31);
            return ret;
        };
        imports.wbg.__wbindgen_init_externref_table = function() {
            const table = wasm.__wbindgen_export_2;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
            ;
        };
        imports.wbg.__wbindgen_is_function = function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        };
        imports.wbg.__wbindgen_is_object = function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        };
        imports.wbg.__wbindgen_is_string = function(arg0) {
            const ret = typeof(arg0) === 'string';
            return ret;
        };
        imports.wbg.__wbindgen_is_undefined = function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        };
        imports.wbg.__wbindgen_memory = function() {
            const ret = wasm.memory;
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

        if (typeof module_or_path === 'undefined' && typeof script_src !== 'undefined') {
            module_or_path = script_src.replace(/\.js$/, '_bg.wasm');
        }
        const imports = __wbg_get_imports();

        if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
            module_or_path = fetch(module_or_path);
        }

        __wbg_init_memory(imports);

        const { instance, module } = await __wbg_load(await module_or_path, imports);

        return __wbg_finalize_init(instance, module);
    }

    wasm_bindgen = Object.assign(__wbg_init, { initSync }, __exports);

})();
