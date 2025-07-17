import { createBrowserRouter } from "react-router";
import { Layout } from "./layout/Layout";

export const router = createBrowserRouter([
    {
        path: "/",
        Component: Layout,
        children: [],
    },
]);
