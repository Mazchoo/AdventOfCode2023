import ReactDOM from "react-dom/client";
import React from "react";
import { HelmetProvider } from "react-helmet-async";
import Day5 from "./day5";
import { HeadLinks } from "../header";

const root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(
  <HelmetProvider>
    <HeadLinks />
    <Day5 />
  </HelmetProvider>
);