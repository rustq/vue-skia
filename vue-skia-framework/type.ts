import {
    ComponentInternalInstance,
} from 'vue';
import { SoftSkiaWASM } from '../soft-skia-wasm/pkg/';

export type ComponentInternalInstanceWithSoftSkiaWASM = ComponentInternalInstance & {
    ssw: SoftSkiaWASM;
    _ssw_id: number;
    _ssw_grouped?: (instance: ComponentInternalInstanceWithSoftSkiaWASM) => void;
    _ssw_batchDraw?: () => void;
}

declare global {
    var ssw: { SoftSkiaWASM: typeof SoftSkiaWASM };
    var core: SoftSkiaWASM;
}