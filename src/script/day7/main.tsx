import ReactDOM from "react-dom/client";
import React from "react";
import { HelmetProvider } from "react-helmet-async";
import Day7 from "./day7";
import { HeadLinks } from "../header";

const root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(
  <HelmetProvider>
    <HeadLinks />
    <Day7 />
  </HelmetProvider>
);