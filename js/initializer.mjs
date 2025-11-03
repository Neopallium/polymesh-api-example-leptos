//import { threads } from 'wasm-feature-detect';

export default function myInitializer() {
    return {
        onStart: () => {
            //console.log("Loading...");
            //console.time("trunk-initializer");
        },
        onProgress: ({ current, total }) => {
            /*
            if (!total) {
                console.log("Loading...", current, "bytes");
            } else {
                console.log("Loading...", Math.round((current / total) * 100), "%")
            }
            */
        },
        onComplete: () => {
            //console.log("Loading... done!");
            //console.timeEnd("trunk-initializer");
        },
        onSuccess: async (wasm) => {
            console.log("Loading... successful!");
            console.log("WebAssembly: ", wasm);
            /*
            if (!(await threads())) {
                console.log("WASM Threads not supported, skipping thread pool initialization");
                return;
            }
            */
            console.log("Initializing WASM Thread Pool with 10 threads...");
            await wasm.initThreadPool(navigator.hardwareConcurrency);
            console.log("WASM Thread Pool initialized");
        },
        onFailure: (error) => {
            console.warn("Loading... failed!", error);
        }
    }
};
