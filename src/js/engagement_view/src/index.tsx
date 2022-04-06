import React from "react";
import "./index.css";

import {ThemeProvider } from "@mui/material/styles";

import App from "./App";
import theme from "./theme";
import * as serviceWorker from "./serviceWorker";
import {createRoot} from "react-dom/client";

const container = document.getElementById("root");
const root =
    createRoot(container);

root.render(
    <React.StrictMode>
        <ThemeProvider theme={theme}>
            <App />
        </ThemeProvider>
    </React.StrictMode>
);

serviceWorker.unregister();
