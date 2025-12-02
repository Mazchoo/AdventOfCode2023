import ReactDOM from "react-dom/client";
import React from "react";
import { HelmetProvider } from "react-helmet-async";
import Day2 from "./day2";
import { HeadLinks } from "../header";

const root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(
  <HelmetProvider>
    <HeadLinks />
    <Day2 />
  </HelmetProvider>
);