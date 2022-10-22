import * as React from "react";
import {Form, Slider} from "antd";
import {InputProps, withCanvas} from "./with-canvas";

function CleanComponent({value, onChange}: InputProps<{ x: number, y: number, w: number, h: number }>) {
    return <Form>
        <Form.Item label="X">
            <Slider value={value.x} min={0} max={1000} onChange={x => onChange({...value, x})}/>
        </Form.Item>
        <Form.Item label="Y">
            <Slider value={value.y} min={0} max={1000} onChange={y => onChange({...value, y})}/>
        </Form.Item>
        <Form.Item label="Width">
            <Slider value={value.w} min={0} max={1000} onChange={w => onChange({...value, w})}/>
        </Form.Item>
        <Form.Item label="Height">
            <Slider value={value.h} min={0} max={1000} onChange={h => onChange({...value, h})}/>
        </Form.Item>
    </Form>;
}

export const ClearPage = withCanvas(
    {x: 250, y: 250, w: 500, h: 500},
    CleanComponent,
    (context) => {
        const frame = context.frame_with_size(100, 100);
        frame.clear(0.3, 0.3, 0, 1);
        return ({x, y, w, h}, width, height) => {
            context.clear(
                new Int32Array([0, 0, width, height]),
                new Float32Array([0, 0.3, 0.3, 1])
            );

            context.draw_frame_1(frame, new Int32Array([width * x / 1000, height * y / 1000, width * w / 1000, height * h / 1000]));
        }
    });
