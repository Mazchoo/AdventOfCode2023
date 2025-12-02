import React from "react";
import { Helmet } from "react-helmet-async";

export const HeadLinks = () => (
  <Helmet>
    <link rel="icon" type="image/x-icon" href="../images/polar.png" />
    <link rel="stylesheet" href="../bulma.min.css" type="text/css" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta charSet="utf-8" />
  </Helmet>
);