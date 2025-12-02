import ReactDOM from "react-dom/client";
import React from "react";
import { HelmetProvider } from "react-helmet-async";
import Day6 from "./day6";
import { HeadLinks } from "../header";

const root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(
  <HelmetProvider>
    <HeadLinks />
    <Day6 />
  </HelmetProvider>
);