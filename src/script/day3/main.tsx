import ReactDOM from "react-dom/client";
import React from "react";
import { HelmetProvider } from "react-helmet-async";
import Day3 from "./day3";
import { HeadLinks } from "../header";

const root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(
  <HelmetProvider>
    <HeadLinks />
    <Day3 />
  </HelmetProvider>
);