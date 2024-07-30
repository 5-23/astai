import React, { StrictMode } from "react";

import { BrowserRouter, Routes, Route } from 'react-router-dom';
import ReactDOM from "react-dom/client";
import Index from "./App";
import Setting from "./Setting";

ReactDOM.createRoot(document.getElementById("root")).render(

    <BrowserRouter>
        <Routes>
            <Route path="/" element={<Index />}></Route>
            <Route path="/settings" element={<Setting />}></Route>
        </Routes>
    </BrowserRouter>
);
