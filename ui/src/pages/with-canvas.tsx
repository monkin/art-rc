import * as React from "react";
import {ComponentType, useCallback, useEffect, useRef, useState} from "react";
import {Context} from "../../../pkg";

function nextFrame(): Promise<void> {
    return new Promise(resolve => requestAnimationFrame(() => resolve()));
}

export interface InputProps<T> {
    value: T;
    onChange: (value: T) => void;
}

export function withCanvas<T>(
    initial: T,
    Input: ComponentType<InputProps<T>>,
    view: (context: Context) => (value: T, width: number, height: number) => void
) {
    return () => {
        const [value, setValue] = useState(initial);
        const ref = useRef(value);
        const onChange = useCallback((v: T) => {
            ref.current = v;
            setValue(v);
        }, []);
        const container = useRef<HTMLDivElement>(null);
        const canvas = useRef<HTMLCanvasElement>(null);

        useEffect(() => {
            let disposed = false;
            const context = new Context(canvas.current!);
            const render = view(context);

            let width = 0;
            let height = 0;

            (async (container: HTMLDivElement, canvas: HTMLCanvasElement) => {
                while (!disposed) {
                    const newWidth = container.clientWidth * devicePixelRatio;
                    const newHeight = container.clientHeight * devicePixelRatio;
                    if (width !== newWidth && height !== newHeight) {
                        canvas.width = width = newWidth;
                        canvas.height = height = newHeight;
                    }
                    render(ref.current, width, height);
                    await nextFrame();
                }
            })(container.current!, canvas.current!);

            return () => {
                context.free();
                disposed = true;
            };
        }, []);

        return <>
            <div className="input-container">
                <Input value={value} onChange={onChange}/>
            </div>
            <div ref={container} className="canvas-container">
                <canvas className="canvas" ref={canvas}/>
            </div>
        </>;
    };
}