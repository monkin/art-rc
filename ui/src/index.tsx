import * as React from "react";
import {createRoot} from "react-dom/client";
import {Menu, MenuProps} from "antd";

import "antd/dist/antd.css";
import "./index.css";

const menuItems: Required<MenuProps>['items'] = [
    {label: "Clear", key: 'clear'},
    {label: "Wave Pencil", key: 'wave-pencil'}
];

(async () => {
    await new Promise(resolve =>
        document.addEventListener("DOMContentLoaded", resolve)
    );

    const {ClearPage} = await import("./pages");

    const App = () => {
        return (
            <div className="layout">
                <Menu className="layout-menu" mode="inline" items={menuItems}/>
                <div className="layout-content">
                    <ClearPage/>
                </div>
            </div>
        );
    };

    createRoot(document.getElementById("root") as Element).render(<App/>);
})();